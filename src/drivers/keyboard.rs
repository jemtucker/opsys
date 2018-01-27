use io::Port;

use schedule::bottom_half::BottomHalf;

use kernel::kget;

static CHARS: [u8; 59] = *b"??1234567890-=\x08?qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";
static CHARS_SHIFT: [u8; 59] = *b"??!@#$%^&*()_+??QWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?";

/// Basic Keyboard driver
pub struct Keyboard {
    port: Port,
    caps: bool,
    shift: u8,
    ctrl: u8,
    alt: u8,
}

impl Keyboard {
    /// Construct a new `Keyboard` for port 0x60.
    ///
    /// Initializes all flags to 0 or false.
    pub const fn new() -> Keyboard {
        Keyboard {
            port: Port::new(0x60),
            caps: false,
            shift: 0,
            ctrl: 0,
            alt: 0,
        }
    }

    /// Get the ASCII character for keycode `code`
    ///
    /// Returns `None` if the code does not map to a valid character.
    fn get_char(&self, code: u8) -> Option<char> {
        if code < 60 {
            let c = if self.caps ^ (self.shift > 0) {
                CHARS_SHIFT[code as usize] as char
            } else {
                CHARS[code as usize] as char
            };

            Some(c)
        } else {
            None
        }
    }

    fn handle_keypress(&mut self) {
        // Get the code of the keypress
        let code = unsafe {
            let c = self.port.read();

            if c == 0xEA {
                self.port.read()
            } else {
                c
            }
        };

        match code {
            0x2A | 0x36 => self.shift += 1,
            0xAA | 0xB6 => self.shift -= 1,
            0x1D => self.ctrl += 1,
            0x9D => self.ctrl -= 1,
            0x38 => self.alt += 1,
            0xB8 => self.alt -= 1,
            0x3A => self.caps = !self.caps,
            _ => {
                // Print the char to the console if valid
                match self.get_char(code) {
                    Some(c) => kprint!("{}", c),
                    None => (),
                }
            }
        }
    }
}

pub struct KeyboardBottomHalf {}

impl KeyboardBottomHalf {
    pub fn new() -> KeyboardBottomHalf {
        KeyboardBottomHalf {}
    }
}

impl BottomHalf for KeyboardBottomHalf {
    fn execute(&mut self) {
        let driver = unsafe { &mut *kget().keyboard.get() };
        driver.handle_keypress();
    }
}
