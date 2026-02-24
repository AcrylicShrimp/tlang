use crate::{BytePos, Span, line_byte_pos_iter::LineBytePosIter};

#[derive(Debug, Clone, Hash)]
pub struct Source {
    pub id: String,
    pub span: Span,
    pub lines: Vec<BytePos>,
    pub content: String,
}

impl Source {
    pub fn new(id: String, base_pos: BytePos, content: String) -> Self {
        #[cfg(debug_assertions)]
        assert!(
            base_pos.index() as usize + content.len() <= u32::MAX as usize,
            "source content is too long"
        );

        let span = Span::new(base_pos, base_pos + BytePos::new(content.len() as u32));
        let lines = std::iter::once(base_pos)
            .chain(LineBytePosIter::new(&content).map(|line_pos| {
                #[cfg(debug_assertions)]
                assert!(
                    line_pos.index() as usize <= u32::MAX as usize,
                    "line byte position is too long"
                );

                base_pos + line_pos
            }))
            .collect();

        Self {
            id,
            span,
            lines,
            content,
        }
    }

    fn find_line_index(&self, pos: BytePos) -> Option<usize> {
        if !self.span.contains_pos(pos) {
            return None;
        }

        match self.lines.binary_search(&pos) {
            Ok(line) => Some(line),
            Err(line) => Some(line - 1),
        }
    }

    pub fn find_line_lo(&self, pos: BytePos) -> Option<BytePos> {
        let line = self.find_line_index(pos)?;
        Some(self.lines[line])
    }

    pub fn find_line_span(&self, pos: BytePos) -> Option<Span> {
        let line = self.find_line_index(pos)?;
        let line_lo = self.lines[line];
        let line_hi = self.lines.get(line + 1).copied().unwrap_or(self.span.hi);
        Some(Span::new(line_lo, line_hi))
    }

    pub fn lookup_char_pos(&self, pos: BytePos) -> Option<(u32, u32)> {
        let line = self.find_line_index(pos)?;
        let line_lo = self.lines[line];
        let line_span = Span::new(line_lo, pos);
        let line_prefix_content = self.slice_content(line_span)?;
        let line_prefix_char_count = line_prefix_content.chars().count();
        Some((line as u32, line_prefix_char_count as u32))
    }

    pub fn slice_content(&self, span: Span) -> Option<&str> {
        if !self.span.contains_span(span) {
            return None;
        }

        let local_lo = (span.lo.index() - self.span.lo.index()) as usize;
        let local_hi = (span.hi.index() - self.span.lo.index()) as usize;

        #[cfg(debug_assertions)]
        {
            assert!(
                self.content.is_char_boundary(local_lo),
                "content is not char boundary"
            );
            assert!(
                self.content.is_char_boundary(local_hi),
                "content is not char boundary"
            );
        }

        Some(&self.content[local_lo..local_hi])
    }
}
