mod interprate;
mod combinators;
mod tests;
mod string_utils;
mod run;

use crate::run::eval_sub_ctx;

fn main() {

    let s = "S(KS)Kxyz".to_string();

    println!("{}", eval_sub_ctx(s));




//    let s = "SSSXYZ".to_string();
//
//    let t = eval_first(s);
//
//    dbg!(&t);
//
//    let u = eval_first(t);
//
//    dbg!(&u);


}
