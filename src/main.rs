//#![feature(test)]
//
//extern crate test;

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
use crate::generate_combinator::gen_programs_brute_force;

use std::thread;
use std::time::Duration;


fn main() {

    //let a: Vec<usize> = (0..7).map(|x| 2 * x).map(|x| {
    //    dbg!(make_combinators(x), x);
    //    make_combinators(x).iter().filter(|s| has_matching_parens(&s[..]))
    //        .count()
    //}).collect();

    for i in 0..7 {
        for p in gen_programs_brute_force(i).into_iter()
        .map(|x| x + "XYZ").collect::<Vec<String>>() {
            println!("Start: {}", &p);
            println!("\tEvald: {}", eval_sub_ctx(p));
        }
    }


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

//#[cfg(test)]
//mod benchmarks {
//    use super::*;
//    use crate::generate_combinator::{ simple_cart_prod, simple_cart_prod_tail };
//    use test::Bencher;
//
//
//    #[bench]
//    fn bench_recurse_at_end(b: &mut Bencher) {
//        b.iter(|| simple_cart_prod(12))
//    }
//
//    #[bench]
//    fn bench_recurse_at_start(b: &mut Bencher) {
//        b.iter(|| simple_cart_prod_tail(12))
//    }
//}
