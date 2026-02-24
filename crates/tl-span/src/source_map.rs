use crate::{BytePos, Source, Span};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Default, Debug, Clone)]
pub struct SourceMap {
    sources: Vec<Arc<Source>>,
    source_indices_by_id: BTreeMap<String, usize>,
}

impl SourceMap {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entire_lo(&self) -> BytePos {
        BytePos::new(0)
    }

    pub fn entire_hi(&self) -> BytePos {
        self.sources
            .last()
            .map(|source| source.span.hi)
            .unwrap_or_default()
    }

    pub fn entire_span(&self) -> Span {
        Span::new(self.entire_lo(), self.entire_hi())
    }

    pub fn add_source(&mut self, id: String, content: String) -> Arc<Source> {
        let base_pos = self
            .sources
            .last()
            .map(|source| source.span.hi)
            .unwrap_or_default();

        let source = Source::new(id.clone(), base_pos, content);
        let source = Arc::new(source);
        let source_index = self.sources.len();

        self.sources.push(source.clone());
        self.source_indices_by_id.insert(id, source_index);

        source
    }

    pub fn find_source_by_id(&self, id: impl AsRef<str>) -> Option<Arc<Source>> {
        let id = id.as_ref();
        let source_index = self.source_indices_by_id.get(id)?;
        Some(self.sources[*source_index].clone())
    }

    fn find_source_index(&self, pos: BytePos) -> Option<usize> {
        if !self.entire_span().contains_pos(pos) {
            return None;
        }

        match self
            .sources
            .binary_search_by_key(&pos, |source| source.span.lo)
        {
            Ok(source) => Some(source),
            Err(source) => Some(source - 1),
        }
    }

    pub fn find_source_by_pos(&self, pos: BytePos) -> Option<Arc<Source>> {
        let source = self.find_source_index(pos)?;
        Some(self.sources[source].clone())
    }

    pub fn find_source_by_span(&self, span: Span) -> Option<Arc<Source>> {
        let lo_source = self.find_source_index(span.lo)?;
        let hi_source = self.find_source_index(span.hi)?;

        if lo_source != hi_source {
            return None;
        }

        Some(self.sources[lo_source].clone())
    }
}
