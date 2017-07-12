mod task;
mod task_context;

/// Task ID for the system idle task
pub const TID_SYSTEMIDLE: u32 = 0;

/// Task ID for the `BottomHalf` processing daemon
pub const TID_BOTTOMHALFD: u32 = 1;

pub use self::task::Task;
pub use self::task::TaskStatus;
pub use self::task::TaskPriority;
pub use self::task_context::TaskContext;
