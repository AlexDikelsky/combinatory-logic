#![feature(test)]

extern crate test;

mod interprate;
mod combinators;
mod tests;
mod string_utils;
mod run;
mod generate_combinator;

use crate::run::eval_sub_ctx;
//use crate::generate_combinator::cart_with_matched_parens;
use crate::generate_combinator::simple_cart_prod_tail;
use string_utils::has_matching_parens;


//fn main() {
//
//    //let a: Vec<usize> = (0..7).map(|x| 2 * x).map(|x| {
//    //    dbg!(make_combinators(x), x);
//    //    make_combinators(x).iter().filter(|s| has_matching_parens(&s[..]))
//    //        .count()
//    //}).collect();
//
//
//    dbg!(simple_cart_prod_tail(4));
//
//
////    let s = "SSSXYZ".to_string();
////
////    let t = eval_first(s);
////
////    dbg!(&t);
////
////    let u = eval_first(t);
////
////    dbg!(&u);
//
//
//}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use crate::generate_combinator::{ simple_cart_prod, simple_cart_prod_tail };
    use test::Bencher;


    #[bench]
    fn bench_recurse_at_end(b: &mut Bencher) {
        b.iter(|| simple_cart_prod(12))
    }

    #[bench]
    fn bench_recurse_at_start(b: &mut Bencher) {
        b.iter(|| simple_cart_prod_tail(12))
    }
}
