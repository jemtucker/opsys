mod rtc;
mod keyboard;

pub use self::rtc::Clock;

pub static mut KEYBOARD: keyboard::Keyboard = keyboard::Keyboard::new();
