/// Sleep the CPU untill the next interrupt
macro_rules! halt {
    () => {
        unsafe { asm!("hlt" :::: "intel"); }
    }
}

/// Sleep the CPU until the next interrupt, when resumed sleep immediately again.
macro_rules! hang {
    () => {
        loop { halt!(); }
    }
}
