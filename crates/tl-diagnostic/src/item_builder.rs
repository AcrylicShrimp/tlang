use crate::{DiagnosticItem, DiagnosticLevel, DiagnosticSubItem};
use tl_span::Span;

pub struct DiagnosticItemBuilder {
    level: DiagnosticLevel,
    span: Span,
    message: String,
    sub_items: Vec<DiagnosticSubItem>,
}

impl DiagnosticItemBuilder {
    pub fn note(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            span,
            message: message.into(),
            sub_items: vec![],
        }
    }

    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            span,
            message: message.into(),
            sub_items: vec![],
        }
    }

    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            span,
            message: message.into(),
            sub_items: vec![],
        }
    }

    pub fn add_sub_note(self, span: Span, message: impl Into<String>) -> Self {
        let mut sub_items = self.sub_items;

        sub_items.push(DiagnosticSubItem {
            level: DiagnosticLevel::Note,
            span,
            message: message.into(),
        });

        Self {
            level: self.level,
            span: self.span,
            message: self.message,
            sub_items,
        }
    }

    pub fn add_sub_warning(self, span: Span, message: impl Into<String>) -> Self {
        let mut sub_items = self.sub_items;

        sub_items.push(DiagnosticSubItem {
            level: DiagnosticLevel::Warning,
            span,
            message: message.into(),
        });

        Self {
            level: self.level,
            span: self.span,
            message: self.message,
            sub_items,
        }
    }

    pub fn add_sub_error(self, span: Span, message: impl Into<String>) -> Self {
        let mut sub_items = self.sub_items;

        sub_items.push(DiagnosticSubItem {
            level: DiagnosticLevel::Error,
            span,
            message: message.into(),
        });

        Self {
            level: self.level,
            span: self.span,
            message: self.message,
            sub_items,
        }
    }

    pub fn build(self) -> DiagnosticItem {
        DiagnosticItem {
            level: self.level,
            span: self.span,
            message: self.message,
            sub_items: self.sub_items,
        }
    }
}
