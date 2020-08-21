use crate::combinators::{S, K, I};
use crate::string_utils::{ delete_matching_paren, tail };

#[test]
fn by_hand() {
    let a = S("(SS)xyz".to_string());
    dbg!(&a);

    let b = delete_matching_paren(tail(a).unwrap());
    dbg!(&b);
    let c = S(tail(b).unwrap());
    dbg!(&c);

    let d = S(tail(c).unwrap());
    dbg!(&d);

    let e = delete_matching_paren(tail(d).unwrap());
    dbg!(&e);

    // This is not the ideal expeced value, there are
    // some unnecisary parenthesis
    // Best is
    // xyz(y(xy)z)
    let expected = "xyz((y(xy))z)".to_string();

    println!("{}\n{}", e, expected);

    assert!(e == expected);
}
