use tl_diagnostic::DiagnosticItem;
use tl_lexer::token_iter;
use tl_parser::{AstModule, Cursor, Parser};
use tl_span::Source;

#[derive(Debug, Clone)]
pub struct CompileResult {
    pub parsed: AstModule,
    pub diagnostics: Vec<DiagnosticItem>,
}

pub fn compile(source: &Source) -> CompileResult {
    let mut diagnostics = Vec::new();

    let parse_result = parse(source);
    diagnostics.extend(parse_result.diagnostics);

    CompileResult {
        parsed: parse_result.parsed,
        diagnostics,
    }
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub parsed: AstModule,
    pub diagnostics: Vec<DiagnosticItem>,
}

pub fn parse(source: &Source) -> ParseResult {
    let cursor = Cursor::new(source.span.hi, token_iter(source));
    let mut parser = Parser::new(cursor);

    let parsed = parser.parse_module();
    let diagnostics = parser.into_diagnostics();

    ParseResult {
        parsed,
        diagnostics,
    }
}
