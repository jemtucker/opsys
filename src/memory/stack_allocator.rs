use collections::LinkedList;

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
    allocated: LinkedList<Stack>,
    /// List of free `Stack` objects
    free: LinkedList<Stack>,
    /// The next page to allocate
    next_page: Page,
}

impl StackAllocator {
    /// Constructs a `StackAllocator` with empty allocated and free lists.
    pub fn new(starting_page: Page) -> StackAllocator {
        StackAllocator {
            allocated: LinkedList::new(),
            free: LinkedList::new(),
            next_page: starting_page,
        }
    }

    /// Allocates a new `Stack`.
    ///
    /// If no `Stack` is available on the free list DEFAULT_STACK_SIZE_PAGES will be mapped along
    /// with a further guard page.
    pub fn allocate(&mut self,
                    table: &mut ActivePageTable,
                    allocator: &mut AreaFrameAllocator)
                    -> Stack {
        // If we have a free stack just return that.
        if !self.free.is_empty() {
            let stack = self.free.pop_front().unwrap();
            self.allocated.push_front(stack);
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
        self.allocated.push_front(stack);
        stack
    }

    fn allocate_pages(&mut self,
                      num: u8,
                      table: &mut ActivePageTable,
                      allocator: &mut AreaFrameAllocator)
                      -> usize {
        // Allocating zero pages is silly
        assert!(num != 0);

        let start = self.allocate_page(table, allocator);
        for _ in 1..num {
            self.allocate_page(table, allocator);
        }

        // Return the start address of the page we just mapped
        start
    }

    fn allocate_page(&mut self,
                     table: &mut ActivePageTable,
                     allocator: &mut AreaFrameAllocator)
                     -> usize {
        let next_page = self.next_page.next_page();
        let page = self.next_page;
        self.next_page = next_page;

        // Map the new page and return the start address
        table.map(page, paging::WRITABLE, allocator);
        page.start_address()
    }
}
