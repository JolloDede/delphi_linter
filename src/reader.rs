struct ReaderIter {
    chars: Vec<char>,
    i: usize,
    row: usize,
    col: usize,
}

impl ReaderIter {
    fn new(content: String) -> Self {
        let charas: Vec<char> = content.chars().collect();
        ReaderIter {
            chars: charas,
            i: 0,
            row: 0,
            col: 0,
        }
    }

    fn next(&mut self) -> Option<&char> {
        if self.i != 0 {
            self.i += 1;
        }
        self.col += 1;

        if let Some(c) = self.chars.get(self.i) {
            if *c == '\n' {
                self.row += 1;
                self.col = 0;
            }
            return Some(c);
        }

        return None;
    }

    fn peek(&mut self) -> Option<&char> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, index: usize) -> Option<&char> {
        self.chars.get(self.i + index)
    }

    fn advance_by(&mut self, index: usize) {
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
        let mut reader = ReaderIter::new(String::from(""));

        assert_eq!(reader.i, 0);
        let c = reader.next();

        assert_eq!(c, None);
    }

    #[test]
    fn col() {
        let mut reader = ReaderIter::new(String::from("1"));

        let _ = reader.next();

        assert_eq!(reader.i, 0);
        assert_eq!(reader.col, 1);
    }

    #[test]
    fn row() {
        let mut reader = ReaderIter::new(String::from("\n"));

        let _ = reader.next();

        assert_eq!(reader.col, 0);
        assert_eq!(reader.row, 1);
    }
}
