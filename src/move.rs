use crate::*;

#[derive(PartialEq)]
pub struct Move {
    pub start: u8,
    pub end:   u8,
}

impl Move {
    pub fn display(&self) -> String {
        return format!(
            "{}{}{}{}",
            (&self.start % 8 + 97) as char,
            &self.start/8 + 1,
            (&self.end % 8 + 97) as char,
            &self.end/8 + 1
        )
    }
}