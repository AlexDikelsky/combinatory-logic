use crate::run::eval_sub_ctx;

#[test]
fn i() {
    let c = eval_sub_ctx("SIKx".to_string());
    let expected = "x(Kx)".to_string();

    dbg!(&c, &expected);
    assert!(c == expected);
}

#[test]
fn ii() {
    let c = eval_sub_ctx("SSKxy".to_string());
    let expected = "xyx".to_string();

    dbg!(&c, &expected);
    assert!(c == expected);
}

#[test]
fn iii() {
    let c = eval_sub_ctx("S(SK)xy".to_string());
    let expected = "xy".to_string();

    dbg!(&c, &expected);
    assert!(c == expected);
}

#[test]
fn iv() {
    let c = eval_sub_ctx("S(KS)Sxyz".to_string());
    let expected = "x(yz)(z(yz))".to_string();

    dbg!(&c, &expected);
    assert!(c == expected);
}



