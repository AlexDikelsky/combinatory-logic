use crate::combinators::{S, K, I};
use crate::string_utils::{ delete_matching_paren, tail, head };

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
