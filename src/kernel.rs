use alloc::boxed::Box;

use core::cell::UnsafeCell;

use schedule::Scheduler;
use memory::MemoryManager;

use drivers;

// The main Kernel pointer, providing access to key objects
pub static mut PKERNEL: Option<&'static mut Kernel> = None;

/// Initialise global kernel objects.
///
/// Kernel objects are accessible through the `kget()` function.
pub fn init(memory_manager: MemoryManager) {
    unsafe {
        PKERNEL = Some(&mut *Box::into_raw(box Kernel::new(memory_manager)));
    }

    let scheduler = unsafe { &mut *kget().scheduler.get() };
    let mut mm = unsafe { &mut *kget().memory_manager.get() };

    scheduler.new_task(&mut mm, hello);
    scheduler.new_task(&mut mm, world);
}

fn hello() {
    unsafe {
        loop {
            ::vga_buffer::print_error(format_args!("Hello"));

            // Sort of sleep
            asm!("hlt" :::: "intel" : "volatile");
        }
    };
}

fn world() {
    unsafe {
        loop {
            ::vga_buffer::print_error(format_args!("World"));

            // Sort of sleep
            asm!("hlt" :::: "intel" : "volatile");
        }
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

    // Drivers
    pub clock: UnsafeCell<drivers::Clock>,
    pub keyboard: UnsafeCell<drivers::Keyboard>,
}

impl Kernel {
    pub fn new(mut memory_manager: MemoryManager) -> Kernel {
        Kernel {
            scheduler: UnsafeCell::new(Scheduler::new(&mut memory_manager)),
            memory_manager: UnsafeCell::new(memory_manager),

            // Drivers
            clock: UnsafeCell::new(drivers::Clock::new()),
            keyboard: UnsafeCell::new(drivers::Keyboard::new()),
        }
    }
}
