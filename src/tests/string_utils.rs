use crate::string_utils::{ delete_matching_paren, tail, has_matching_parens };

#[test]
fn delete_paren_1() {
    let a = delete_matching_paren("asdf)".to_string());
    let expected = "asdf".to_string();

    assert!(a == expected);
}

#[test]
fn delete_paren_2() {
    let a = delete_matching_paren("as()df)".to_string());
    let expected = "as()df".to_string();

    assert!(a == expected);
}

#[test]
#[should_panic]
fn delete_paren_3() {
    delete_matching_paren("asdf".to_string());
}

#[test]
fn delete_paren_4() {
    let a = delete_matching_paren(")sdf".to_string());
    let expected = "sdf".to_string();

    assert!(a == expected);
}

#[test]
#[should_panic]
fn delete_paren_5() {
    delete_matching_paren("(())asdf(())".to_string());
}

#[test]
fn delete_paren_6() {
    let a = delete_matching_paren(")a)a".to_string());
    let expected = "a)a".to_string();

    assert!(a == expected);
}

#[test]
fn tail_1() {
    let a = tail("abc".to_string());
    let expected = Some("bc".to_string());

    assert!(a == expected);
}

#[test]
fn tail_2() {
    let a = tail("a".to_string());
    let expected = Some("".to_string());

    assert!(a == expected);
}

#[test]
fn tail_3() {
    let a = tail("".to_string());
    let expected = None;

    assert!(a == expected);
}

#[test]
fn match_1() {
    assert!(!has_matching_parens("("));
    assert!(!has_matching_parens(")("));
    assert!(!has_matching_parens("))(()("));
}

#[test]
fn match_2() {
    assert!(has_matching_parens(""));
    assert!(has_matching_parens("()"));
    assert!(has_matching_parens("()asdf()ssdf"));
    assert!(has_matching_parens("(((()    ))()sdfasdfsadf)()"))
}
