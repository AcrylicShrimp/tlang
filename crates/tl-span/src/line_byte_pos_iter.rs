use crate::BytePos;

#[derive(Debug, Clone, Copy)]
pub struct LineBytePosIter<'a> {
    s: &'a str,
    index: usize,
}

impl<'a> LineBytePosIter<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s, index: 0 }
    }

    fn find_line_end(&mut self) -> Option<usize> {
        let bytes = &self.s.as_bytes()[self.index..];

        for (i, b) in bytes.iter().copied().enumerate() {
            if b == b'\n' {
                return Some(self.index + i + 1);
            }

            if b == b'\r' && bytes.get(i + 1).copied() == Some(b'\n') {
                return Some(self.index + i + 2);
            }
        }

        None
    }
}

impl<'a> Iterator for LineBytePosIter<'a> {
    type Item = BytePos;

    fn next(&mut self) -> Option<Self::Item> {
        let line_end = self.find_line_end()?;

        #[cfg(debug_assertions)]
        assert!(
            line_end <= u32::MAX as usize,
            "line byte position is too long"
        );

        let pos = BytePos::new(line_end as u32);
        self.index = line_end;

        Some(pos)
    }
}
