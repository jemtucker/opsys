use io::Port;
use vga_buffer;

static CHARS: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?";
static CHARS_SHIFT: [u8; 59] = *b"??!@#$%^&*()_+??QWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?";

pub struct Keyboard {
    port: Port,
    caps: bool,
    shift: u8,
    ctrl: u8,
    alt: u8
}

impl Keyboard {
    pub const fn new() -> Keyboard {
        Keyboard {
            port: Port::new(0x60),
            caps: false,
            shift: 0,
            ctrl: 0,
            alt: 0
        }
    }

    pub fn handle_keypress(&mut self) {
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
            _ => { let _ = self.get_char(code); }
        }
    }

    fn get_char(&self, code: u8) -> Option<char> {
        if code < 60 {
            let c = if self.caps ^ (self.shift > 0) {
                CHARS_SHIFT[code as usize] as char
            } else {
                CHARS[code as usize] as char
            };

            unsafe { kprint!("{}", c); }

            Some(c)
        } else {
            None
        }
    }
}