pub struct Reader {
    chars: Vec<char>,
    i: usize,
    pub row: usize,
    pub col: usize,
}

impl Reader {
    pub fn new(content: String) -> Self {
        let charas: Vec<char> = content.chars().collect();
        Reader {
            chars: charas,
            i: 0,
            row: 1,
            col: 1,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(c) = self.chars.get(self.i) {
            if *c == '\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.i += 1;
            return Some(*c);
        }

        return None;
    }

    pub fn peek(&mut self) -> Option<char> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&self, offset: usize) -> Option<char> {
        self.chars.get(self.i + offset).copied()
    }

    pub fn advance_by(&mut self, index: usize) {
        for _ in 0..index {
            self.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::*;

    #[test]
    fn empty() {
        let mut reader = Reader::new(String::from(""));

        assert_eq!(reader.i, 0);
        let c = reader.next();

        assert_eq!(c, None);
    }

    #[test]
    fn i() {
        let mut reader = Reader::new(String::from("12"));

        let _ = reader.next();

        assert_eq!(reader.i, 1);
        assert_eq!(reader.col, 2);

        let _ = reader.next();

        assert_eq!(reader.i, 2);
        assert_eq!(reader.col, 3);
    }

    #[test]
    fn col() {
        let mut reader = Reader::new(String::from("1"));

        let _ = reader.next();

        assert_eq!(reader.i, 1);
        assert_eq!(reader.col, 2);
    }

    #[test]
    fn row() {
        let mut reader = Reader::new(String::from("\n"));

        let _ = reader.next();

        assert_eq!(reader.col, 1);
        assert_eq!(reader.row, 2);
    }

    #[test]
    fn advance_by() {
        let mut reader = Reader::new(String::from("1234"));

        let _ = reader.advance_by(1);

        assert_eq!(reader.i, 1);

        let _ = reader.advance_by(1);

        assert_eq!(reader.i, 2);

        let _ = reader.advance_by(2);

        assert_eq!(reader.i, 4);
    }
}
