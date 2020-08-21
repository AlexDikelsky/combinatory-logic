mod interprate;
mod combinators;

use crate::interprate::next_n_terms;
use crate::combinators::{S, K, I};
fn main() {
//    dbg!(S(s));

}

//#[test]
//fn f() {
//    let s1 = "S((SS)X)YZ".to_string();
//    let a = next_term(s1);
//    let b = next_term(a.clone().1.unwrap());
//    let c = next_term(b.clone().1.unwrap());
//   
//    dbg!(a, b, c);
//}
//
//
//
//#[test]
//fn g() {
//    let s1 = "S((SS)X)YZ".to_string();
//    dbg!(next_n_terms(4, s1));
//}

#[test]
fn simple_S() {
    let asdf = "xyz".to_string();
    let sed = S(asdf);

    assert!(sed == "xz(yz)".to_string());
}

#[test]
fn simple_K() {
    let asdf = "xy".to_string();
    let ked = K(asdf);

    assert!(ked == "x".to_string());
}

#[test]
fn simple_I() {
    let asdf = "x".to_string();
    let identity = I(asdf);

    assert!(identity == "x".to_string());
}
