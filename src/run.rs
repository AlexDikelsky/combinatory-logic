use crate::combinators::{S, K, I};
use crate::string_utils::{ delete_matching_paren, tail, head };
use crate::interprate::terms_until_none;

// Evaluate the first character
pub fn eval_first(s: String) -> String {
    let cloned = s.clone();
    let head = head(&s[..]);
    let tail = tail(s);
    match head {
        Some(c) => match c {
            '(' => delete_matching_paren(tail.unwrap()),
            ')' => panic!("Leading char is a close paren"),
            'S' => S(tail.unwrap()),
            'K' => K(tail.unwrap()),
            'I' => I(tail.unwrap()),
            'a'..='z' => cloned,
            'A'..='Z' => cloned,
            _ => panic!("Illegal character"),
        }
        None => {
            "".to_string()
        }
    }
}

pub fn eval_ctx(s: String) -> String {
    let mut old = s.clone();
    let mut new = eval_first(s);

    while old != new {
        old = new;
        new = eval_first(old.clone());
    }

    return new
}

pub fn eval_sub_ctx(s: String) -> String {

    let mut old = s.clone();
    let mut new = eval_ctx(s);

    while old != new {
        old = new;
        new = terms_until_none(old.clone()).iter()
            .map(|t| {
                let inner_str = eval_sub_ctx(t.to_string());
                if inner_str.len() > 1 {
                    "(".to_string() + &inner_str + ")"
                } else {
                    inner_str
                }
            }).collect();
    }

    return new
}
