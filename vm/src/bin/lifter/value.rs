use core::fmt;

pub struct Value(pub u16);

impl Value {
    pub fn is_literal(&self) -> bool {
        self.0 <= 32767
    }

    pub fn is_register(&self) -> bool {
        self.0 > 32767 && self.0 <= 32775
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_literal() {
            write!(f, "\x1b[38;5;2m{}\x1b[0m", self.0)
        } else if self.is_register() {
            let register = self.0 - 32768;
            write!(f, "\x1b[38;5;3m${}\x1b[0m", register)
        } else {
            panic!("Invalid data {}", self.0)
        }
    }
}
