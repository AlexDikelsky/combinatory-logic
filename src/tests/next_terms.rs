use crate::interprate::next_term;
use crate::interprate::next_n_terms;


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
