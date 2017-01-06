use alloc::boxed::Box;
use core::cell::UnsafeCell;

use schedule::Scheduler;
use memory::MemoryManager;

// The main Kernel pointer, providing access to key objects
pub static mut PKERNEL: Option<&'static mut Kernel> = None;

pub fn init(memory_manager: MemoryManager) {
    unsafe {
        PKERNEL = Some(&mut *Box::into_raw(box Kernel::new(memory_manager)));
    }

    let mut scheduler = unsafe { &mut *kget().scheduler.get() };
    // scheduler.schedule(hello_world, 100);
}

fn hello_world() {
    unsafe {
        ::vga_buffer::print_error(format_args!("Hello World!"));
    };
}

pub fn kget() -> &'static Kernel {
    unsafe {
        match PKERNEL {
            Some(&mut ref p) => p,
            None => unreachable!(),
        }
    }
}

pub struct Kernel {
    pub scheduler: UnsafeCell<Scheduler>,
    pub memory_manager: UnsafeCell<MemoryManager>,
}

impl Kernel {
    pub fn new(memory_manager: MemoryManager) -> Kernel {
        Kernel {
            scheduler: UnsafeCell::new(Scheduler::new()),
            memory_manager: UnsafeCell::new(memory_manager),
        }
    }
}
