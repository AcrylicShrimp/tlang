#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticLevel {
    Note,
    Warning,
    Error,
}

impl DiagnosticLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticLevel::Note => "note",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Error => "error",
        }
    }
}
