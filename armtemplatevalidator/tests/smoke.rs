extern crate templatevalidator;
use templatevalidator::*;

#[test]
fn round_bracket_in_a_middle() {
    let line = "\"ygu(qwe)gr(fak)flas\"";
    assert!(line.is_balanced('(', ')'));
}

#[test]
fn round_bracket_at_the_end() {
    let line = "\"(wq()g(())gq)\"";
    assert!(line.is_balanced('(', ')'));
}

#[test]
fn figure_bracket_in_a_middle() {
    let line = "h56je{ngje}mka";
    assert!(line.is_balanced('{', '}'));
}

#[test]
fn figure_bracket_at_the_end() {
    let line = "ga{wqggq}bz";
    assert!(line.is_balanced('{', '}'));
}

#[test]
fn figure_bracket_single_at_the_end() {
    let line = "ga{wqggq}bz {";
    assert!(line.is_balanced('{', '}'));
}

#[test]
fn figure_bracket_single() {
    let line = " { ";
    assert!(line.is_balanced('{', '}'));
}

#[test]
fn square_bracket_in_a_middle() {
    let line = "71234[ghg]bb";
    assert!(line.is_balanced('[', ']'));
}

#[test]
fn square_bracket_at_the_end() {
    let line = "[]";
    assert!(line.is_balanced('[', ']'));
}

#[test]
fn square_bracket_single_at_the_end() {
    let line = "asdb[bf] nt [ ";
    assert!(line.is_balanced('[', ']'));
}

#[test]
fn square_bracket_single() {
    let line = "qwgqweg [ ";
    assert!(line.is_balanced('[', ']'));
}

#[test]
fn square_bracket_must_fail() {
    let line = "asdb[bf] nt[ [ ]";
    assert!(!line.is_balanced('[', ']'));
}

#[test]
fn figure_bracket_must_fail() {
    let line = "{}{}{{}{";
    assert!(!line.is_balanced('{', '}'));
}

#[test]
fn round_bracket_must_fail() {
    let line = "(()(())))(())";
    assert!(!line.is_balanced('(', ')'));
}

#[test]
fn quote_check() {
    let line = "awnfke'nkvnj'venakjr'basb[']";
    assert!(line.is_quote_balanced('\''));
}

#[test]
fn double_quote_check() {
    let line = "\"bbf)\"vaa9{[\"781\"";
    assert!(line.is_quote_balanced('"'));
}

#[test]
fn quote_check_must_fail() {
    let line = "'''''";
    assert!(!line.is_quote_balanced('\''));
}

#[test]
fn double_quote_check_must_fail() {
    let line = "\"here\"naa\"";
    assert!(!line.is_quote_balanced('"'));
}

#[test]
fn json_is_valid() {
    let s = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    let errors = Templates::check_json(s);
    assert!(errors.is_empty());
}

#[test]
fn json_is_invalid() {
    let s = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones":
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    let errors = Templates::check_json(s);
    assert!(!errors.is_empty());
}
