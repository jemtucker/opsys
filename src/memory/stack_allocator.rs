use alloc::Vec;

use super::PAGE_SIZE;
use super::paging;
use super::paging::Page;
use super::paging::ActivePageTable;
use super::area_frame_allocator::AreaFrameAllocator;

const DEFAULT_STACK_SIZE_PAGES: u8 = 2;

/// A structre defining memory allocated for use as a kernel stack
#[derive(Clone, Copy, Debug)]
pub struct Stack {
    /// Starting (low) address of the stack
    pub start_address: usize,
    /// Size of the stack in bytes
    pub size: usize,
}

impl Stack {
    /// Return the address at the top of the `Stack`
    pub fn top(&self) -> usize {
        self.start_address - self.size
    }
}

/// Allocator of `Stack` objects.
///
/// The `StackAllocator` manages allocated stacks, maintaining an internal list of both free and
/// allocated stacks. When allocating a `Stack`, if no free stacks are available a new page will be
/// mapped.
pub struct StackAllocator {
    /// List of allocated `Stack` objects that have not yet been free'ed
    allocated: Vec<Stack>,
    /// List of free `Stack` objects
    free: Vec<Stack>,
    /// The next page to allocate
    next_page: Page,
}

impl StackAllocator {
    /// Constructs a `StackAllocator` with empty allocated and free lists.
    pub fn new(starting_page: Page) -> StackAllocator {
        StackAllocator {
            allocated: Vec::new(),
            free: Vec::new(),
            next_page: starting_page,
        }
    }

    /// Allocates a new `Stack`.
    ///
    /// If no `Stack` is available on the free list DEFAULT_STACK_SIZE_PAGES will be mapped along
    /// with a further guard page.
    pub fn allocate(
        &mut self,
        table: &mut ActivePageTable,
        allocator: &mut AreaFrameAllocator,
    ) -> Stack {
        // If we have a free stack just return that.
        if !self.free.is_empty() {
            let stack = self.free.pop().unwrap();
            self.allocated.push(stack);
            return stack;
        }

        // Allocate a guard page by skipping to the next page
        let next_page = self.next_page.next_page();
        self.next_page = next_page;

        // Allocate the stack pages
        let start = self.allocate_pages(DEFAULT_STACK_SIZE_PAGES, table, allocator);
        let size = (DEFAULT_STACK_SIZE_PAGES as usize) * PAGE_SIZE;

        // Store the stack on the allocated list and return a copy
        let stack = Stack {
            start_address: start,
            size: size,
        };

        self.allocated.push(stack);

        kprintln!("Count: {} Allocate: {:?}", self.allocated.len(), stack);
        for s in &self.allocated {
            kprintln!("A: {:?}", s);
        }

        stack
    }

    /// Deallocates a kernel `Stack`.
    ///
    /// The `Stack` is found on the allocated list and moved onto the free list.
    pub fn deallocate(&mut self, stack: &Stack) {
        for s in &self.free {
            kprintln!("Deallocate: {:?}", s);
        }

        let index = self.allocated
            .iter()
            .position(|s| s.start_address == stack.start_address && s.size == stack.size);

        // If the stack can be found then return it otherwise panic!
        match index {
            Some(i) => {
                let freed_stack = self.allocated.swap_remove(i);
                self.free.push(freed_stack);
            }
            None => panic!(
                "Attempt to free a Stack that has not been allocated: {:?}",
                stack
            ),
        }
    }

    fn allocate_pages(
        &mut self,
        num: u8,
        table: &mut ActivePageTable,
        allocator: &mut AreaFrameAllocator,
    ) -> usize {
        // Allocating zero pages is silly
        assert!(num != 0);

        let start = self.allocate_page(table, allocator);
        for _ in 1..num {
            self.allocate_page(table, allocator);
        }

        // Return the start address of the page we just mapped
        start
    }

    fn allocate_page(
        &mut self,
        table: &mut ActivePageTable,
        allocator: &mut AreaFrameAllocator,
    ) -> usize {
        let next_page = self.next_page.next_page();
        let page = self.next_page;
        self.next_page = next_page;

        // Map the new page and return the start address
        table.map(page, paging::WRITABLE, allocator);
        page.start_address()
    }
}
