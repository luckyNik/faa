use faa::cli::{run, split_statements};

#[test]
fn split_complete_and_remainder() {
    let (stmts, rest) = split_statements("CREATE a; INSERT a {1,");
    assert_eq!(stmts, vec!["CREATE a"]);
    assert_eq!(rest.trim(), "INSERT a {1,");
}

#[test]
fn split_multiple_on_one_line() {
    let (stmts, rest) = split_statements("CREATE a;CREATE b;");
    assert_eq!(stmts, vec!["CREATE a", "CREATE b"]);
    assert_eq!(rest.trim(), "");
}

#[test]
fn multiline_statement_and_error_recovery() {
    let input = "CREATE a;\nINSERT a {1,\n 2};\nBOGUS;\nexit;\nCREATE never;\n";
    let mut out = Vec::new();
    run(input.as_bytes(), &mut out).unwrap();
    let out = String::from_utf8(out).unwrap();
    assert!(out.contains("Syntax error"), "{out}");
    assert!(!out.contains("never"), "{out}");
}

#[test]
fn eof_terminates() {
    let mut out = Vec::new();
    run("CREATE a;".as_bytes(), &mut out).unwrap();
}
