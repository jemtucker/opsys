use schedule::Scheduler;
use alloc::boxed::Box;
use core::cell::UnsafeCell;

// The main Kernel pointer, providing access to key objects
pub static mut PKERNEL: Option<&'static mut Kernel> = None;

pub fn init() {
    unsafe {
        PKERNEL = Some(&mut *Box::into_raw(box Kernel::new()));
    }

    let mut scheduler = unsafe { &mut *kget().scheduler.get() };
    scheduler.schedule(ticktock, 10);
}

pub fn ticktock() {
    kprintln!("TickTock!");
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
}

impl Kernel {
    pub fn new() -> Kernel {
        Kernel {
            scheduler: UnsafeCell::new(Scheduler::new())
        }
    }
}