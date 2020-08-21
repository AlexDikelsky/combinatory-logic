mod interprate;
mod combinators;
mod tests;
mod string_utils;
mod run;

use crate::combinators::{S, K, I};
use crate::string_utils::delete_matching_paren;
use crate::run::eval_first;

fn main() {

    let s = "SSSXYZ".to_string();

    let t = eval_first(s);

    dbg!(&t);

    let u = eval_first(t);

    dbg!(&u);


}
