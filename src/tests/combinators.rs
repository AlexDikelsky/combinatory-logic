use crate::combinators::{S, K, I};

#[test]
fn no_extra_args_s() {
    let asdf = "xyz".to_string();
    let sed = S(asdf);
    dbg!(&sed);

    assert!(sed == "xz(yz)".to_string());
}

#[test]
fn no_extra_args_k() {
    let asdf = "xy".to_string();
    let ked = K(asdf);

    dbg!(&ked);

    assert!(ked == "x".to_string());
}

#[test]
fn no_extra_args_i() {
    let asdf = "x".to_string();
    let identity = I(asdf);
        
    dbg!(&identity);

    assert!(identity == "x".to_string());
}


#[test]
fn extra_args_s() {
    let asdf = "xyzp".to_string();
    let sed = S(asdf);
    dbg!(&sed);

    assert!(sed == "xz(yz)p".to_string());
}

#[test]
fn extra_args_k() {
    let asdf = "xyp".to_string();
    let ked = K(asdf);

    dbg!(&ked);

    assert!(ked == "xp".to_string());
}

#[test]
fn extra_args_i() {
    let asdf = "xp".to_string();
    let identity = I(asdf);
        
    dbg!(&identity);

    assert!(identity == "xp".to_string());
}


#[test]
fn not_enough_args_s() {
    let asdf = "xy".to_string();
    let sed = S(asdf);
    dbg!(&sed);

    assert!(sed == "Sxy".to_string());
}

#[test]
fn not_enough_args_k() {
    let asdf = "x".to_string();
    let ked = K(asdf);

    dbg!(&ked);

    assert!(ked == "Kx".to_string());
}

#[test]
fn not_enough_args_i() {
    let asdf = "".to_string();
    let identity = I(asdf);
        
    dbg!(&identity);

    assert!(identity == "I".to_string());
}
