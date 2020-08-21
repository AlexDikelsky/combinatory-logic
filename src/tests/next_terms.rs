use crate::interprate::next_term;
use crate::interprate::next_n_terms;
use crate::interprate::terms_until_none;


#[test]
fn f() {
    let s1 = "S((SS)X)Y".to_string();
    let a = next_term(s1);
    let b = next_term(a.clone().1.unwrap());
    let c = next_term(b.clone().1.unwrap());

    dbg!(&a, &b, &c);
   
    assert!(a.0 == "S".to_string());
    assert!(b.0 == "(SS)X".to_string());
    assert!(c.0 == "Y".to_string());
    assert!(c.1 == None);
}



#[test]
fn g() {
    let s1 = "S((SS)X)YZ".to_string();
    dbg!(next_n_terms(4, s1));
}

#[test]
fn all_s_1() {
    let s = terms_until_none("x(III(II(II)))".to_string());
    let expected = vec!["x".to_string(), "III(II(II))".to_string()];

    dbg!(&s, &expected);
    assert!(s == expected);
}

#[test]
fn all_s_2() {
    let s = terms_until_none("S(KS)Sxyz".to_string());
    let expected = vec![
        "S".to_string(),
        "KS".to_string(),
        "S".to_string(),
        "x".to_string(),
        "y".to_string(),
        "z".to_string(),
    ];
    dbg!(&s, &expected);
    assert!(s == expected);
}

#[test]
fn all_s_3() {
    let s = terms_until_none("".to_string());
    let expected: Vec<String> = vec![];
    dbg!(&s, &expected);
    assert!(s == expected);
}
