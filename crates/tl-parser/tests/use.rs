use tl_lexer::token_iter;
use tl_parser::{AstTopLevelItemKind, AstUseTailKind, Cursor, Parser};
use tl_span::{BytePos, Source};

fn parse_module(input: &str) -> (tl_parser::AstModule, Vec<tl_diagnostic::DiagnosticItem>) {
    let source = Source::new("test".to_owned(), BytePos::new(0), input.to_owned());
    let cursor = Cursor::new(source.span.hi, token_iter(&source));
    let mut parser = Parser::new(cursor);
    let module = parser.parse_module();
    let diagnostics = parser.into_diagnostics();
    (module, diagnostics)
}

#[test]
fn test_use_simple_path() {
    let input = include_str!("cases/use/simple_path.tl");
    let (module, diagnostics) = parse_module(input);

    assert!(diagnostics.is_empty());
    assert_eq!(module.items.len(), 1);

    let item = &module.items[0];
    let use_item = match &item.kind {
        AstTopLevelItemKind::Use(use_item) => use_item,
        AstTopLevelItemKind::ExposeRefFn(_) => panic!("expected use item"),
    };

    assert_eq!(use_item.path.first.name, "core");
    assert_eq!(use_item.path.rest.len(), 1);
    assert_eq!(use_item.path.rest[0].1.name, "printf");
    assert!(use_item.tail.is_none());
}

#[test]
fn test_use_alias() {
    let input = include_str!("cases/use/alias.tl");
    let (module, diagnostics) = parse_module(input);

    assert!(diagnostics.is_empty());
    assert_eq!(module.items.len(), 1);

    let item = &module.items[0];
    let use_item = match &item.kind {
        AstTopLevelItemKind::Use(use_item) => use_item,
        AstTopLevelItemKind::ExposeRefFn(_) => panic!("expected use item"),
    };

    assert_eq!(use_item.path.first.name, "core");
    assert!(use_item.path.rest.is_empty());

    let tail = use_item.tail.as_ref().expect("expected alias tail");
    match &tail.kind {
        AstUseTailKind::As(alias) => assert_eq!(alias.name.name, "my_core"),
        AstUseTailKind::All(_) => panic!("expected alias tail"),
    }
}

#[test]
fn test_use_wildcard() {
    let input = include_str!("cases/use/wildcard.tl");
    let (module, diagnostics) = parse_module(input);

    assert!(diagnostics.is_empty());
    assert_eq!(module.items.len(), 1);

    let item = &module.items[0];
    let use_item = match &item.kind {
        AstTopLevelItemKind::Use(use_item) => use_item,
        AstTopLevelItemKind::ExposeRefFn(_) => panic!("expected use item"),
    };

    let tail = use_item.tail.as_ref().expect("expected wildcard tail");
    match &tail.kind {
        AstUseTailKind::All(_) => {}
        AstUseTailKind::As(_) => panic!("expected wildcard tail"),
    }
}

#[test]
fn test_use_invalid_missing_alias_name_reports_diagnostic() {
    let input = include_str!("cases/use/invalid_missing_alias_name.tl");
    let (_module, diagnostics) = parse_module(input);

    assert!(
        !diagnostics.is_empty(),
        "expected parse diagnostics for invalid use alias"
    );
}
