use faa::command::{Command, SearchQuery};
use faa::error::ParseErrorKind;
use faa::lexer::{TokenKind, tokenize};
use faa::parser::parse;

fn kinds(s: &str) -> Vec<TokenKind> {
    tokenize(s).unwrap().into_iter().map(|t| t.kind).collect()
}

#[test]
fn lexer_basic() {
    assert_eq!(
        kinds("INSERT g {1, -2}"),
        vec![
            TokenKind::Ident("INSERT".into()),
            TokenKind::Ident("g".into()),
            TokenKind::LBrace,
            TokenKind::Number(1),
            TokenKind::Comma,
            TokenKind::Number(-2),
            TokenKind::RBrace,
        ]
    );
}

#[test]
fn lexer_no_spaces() {
    assert_eq!(kinds("{1,-2}").len(), 5);
}

#[test]
fn lexer_rejects_garbage() {
    assert!(matches!(tokenize("CREATE a$").unwrap_err().kind, ParseErrorKind::UnexpectedChar('$')));
    assert!(matches!(tokenize("{-}").unwrap_err().kind, ParseErrorKind::InvalidNumber(_)));
}

#[test]
fn create() {
    assert_eq!(parse("CREATE grades").unwrap(), Command::Create { name: "grades".into() });
}

#[test]
fn keywords_case_insensitive_names_case_sensitive() {
    assert_eq!(parse("create Grades").unwrap(), Command::Create { name: "Grades".into() });
}

#[test]
fn insert() {
    assert_eq!(
        parse("INSERT grades {90, 85, 95}").unwrap(),
        Command::Insert { collection: "grades".into(), set: vec![90, 85, 95] }
    );
}

#[test]
fn print_tree() {
    assert_eq!(parse("PRINT_TREE grades").unwrap(), Command::PrintTree { collection: "grades".into() });
}

#[test]
fn contains() {
    assert_eq!(
        parse("CONTAINS grades {1,2}").unwrap(),
        Command::Contains { collection: "grades".into(), set: vec![1, 2] }
    );
}

#[test]
fn search_plain_and_where() {
    assert_eq!(
        parse("SEARCH g").unwrap(),
        Command::Search { collection: "g".into(), filter: None }
    );
    assert_eq!(
        parse("SEARCH g WHERE INTERSECTS {82, 90}").unwrap(),
        Command::Search { collection: "g".into(), filter: Some(SearchQuery::Intersects(vec![82, 90])) }
    );
    assert_eq!(
        parse("SEARCH g WHERE CONTAINS {85}").unwrap(),
        Command::Search { collection: "g".into(), filter: Some(SearchQuery::Contains(vec![85])) }
    );
    assert_eq!(
        parse("SEARCH g WHERE CONTAINED_BY {1, 2}").unwrap(),
        Command::Search { collection: "g".into(), filter: Some(SearchQuery::ContainedBy(vec![1, 2])) }
    );
}

#[test]
fn errors() {
    assert!(matches!(parse("").unwrap_err().kind, ParseErrorKind::EmptyStatement));
    assert!(matches!(parse("DROP g").unwrap_err().kind, ParseErrorKind::UnknownCommand(_)));
    assert!(matches!(parse("CREATE").unwrap_err().kind, ParseErrorKind::UnexpectedEnd { .. }));
    assert!(matches!(parse("CREATE where").unwrap_err().kind, ParseErrorKind::ReservedWord(_)));
    assert!(matches!(parse("CREATE a b").unwrap_err().kind, ParseErrorKind::TrailingTokens(_)));
    assert!(matches!(parse("INSERT g {1, 2").unwrap_err().kind, ParseErrorKind::UnexpectedEnd { .. }));
    assert!(matches!(parse("INSERT g {1 2}").unwrap_err().kind, ParseErrorKind::UnexpectedToken { .. }));
    assert!(matches!(parse("INSERT g {1,}").unwrap_err().kind, ParseErrorKind::UnexpectedToken { .. }));
    assert!(matches!(parse("INSERT g {1, 1}").unwrap_err().kind, ParseErrorKind::DuplicateElement(_)));
    assert!(matches!(parse("SEARCH g WHERE").unwrap_err().kind, ParseErrorKind::UnexpectedEnd { .. }));
    assert!(matches!(parse("SEARCH g WHERE FOO {1}").unwrap_err().kind, ParseErrorKind::UnexpectedToken { .. }));
}
