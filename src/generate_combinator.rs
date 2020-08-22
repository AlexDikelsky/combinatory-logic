use std::cmp::Ordering;

const COMBINATORS: [char; 3] = [
    'S', 'K', 'I',
];

const COMB_EITHER: [char; 3] = ['S', '(', ')'];
const COMB_OPEN:   [char; 2] = ['S', '('];
const COMB_CLOSE:  [char; 2] = ['S', ')'];

pub fn simple_cart_prod(n: usize) -> Vec<String> {
    if n > 1 {
        COMBINATORS.iter().map(|x| {
            simple_cart_prod(n-1).into_iter() 
                .map(|y| { y + &x.to_string() })
            .collect::<Vec<String>>()
        }).flatten().collect()
    } else {
        COMBINATORS.iter().map(|x| x.to_string()).collect()
    }
}

pub fn simple_cart_prod_tail(n: usize) -> Vec<String> {
    if n > 1 {
        simple_cart_prod_tail(n-1).into_iter().map(|y| {
            COMBINATORS.iter().map(|x| y.clone() + &x.to_string() )
            .collect::<Vec<String>>()
        }).flatten().collect()
    } else {
        COMBINATORS.iter().map(|x| x.to_string()).collect()
    }
}

//pub fn one_matched_paren(n: usize) -> Vec<(bool, String)> {
//    if n > 1 {
//        COMB_EITHER.iter().map(|x| {
//


 
//// Make strings of len n that contain matching parentesis
//// Doesn't work for n=1, but more than that is fine
//pub fn cart_with_matched_parens(n: usize) -> Vec<(usize, String)> {
//    if n > 1 {
//        cart_with_matched_parens(n-1).into_iter().map(
//            |(open_parens, s)| { 
//
//                let remaining_characters = 1 + n - s.len();
//                dbg!(remaining_characters, &s, open_parens);
//                println!();
//
//                match open_parens.cmp(&remaining_characters) {
//                    // There are more parenthesis than there are 'spaces' left
//                    Ordering::Greater => panic!("Too many parens added"),
//
//                    // There are exactly enough spaces for more ')'s, so only
//                    // use ')'s.
//                    Ordering::Equal   => vec![(remaining_characters, s + ")")],
//
//                    // There is room for open parens
//                    Ordering::Less    => {
//                        match open_parens > 0 {
//
//                            // Can add either close or open parens
//                            true  => COMB_EITHER.iter().map(|c| match c {
//                                '(' => (open_parens + 1, s.clone() + &c.to_string()),
//                                ')' => (open_parens - 1, s.clone() + &c.to_string()),
//                                _   => (open_parens, s.clone() + &c.to_string()),
//                            }).collect::<Vec<(usize, String)>>(),
//
//                            // Not parens there right now, so only add open
//                            false => COMB_OPEN.iter().map(|c| match c {
//                                '(' => (1, s.clone() + &c.to_string()),
//                                _   => (0, s.clone() + &c.to_string()),
//                            }).collect::<Vec<(usize, String)>>()
//
//                        }
//                    }
//                }
//            }).flatten().collect()
//    } else {
//        COMB_OPEN.iter().map(|c| match c {
//            '(' => (1, c.to_string()),
//            _ => (0, c.to_string()),
//        }).collect()
//    }
//}
