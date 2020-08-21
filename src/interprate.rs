use crate::string_utils::tail;

// If there are n more terms, return a vector of those terms
// Otherwise, return the original string, and None for the vector
//
// If there are excess characters, return a string in the second tuple place
pub fn next_n_terms(n: usize, s: String) -> (Option<Vec<String>>, Option<String>) {
    let mut terms = vec![];
    let mut none_found = false;
    let mut i = 0;
    let default_str = s.clone();
    let mut cloned_str = s.clone();


    // Loop until just before the last term
    while i < n-1 && !none_found {
        let (new_term, rest_of_str) = next_term(cloned_str.clone());
        terms.push(new_term);

        match rest_of_str {
            Some(x) => { cloned_str = x; },

            None => { none_found = true },
        }; 
        i += 1;
    }

    // The last term's next can be None and it is still okay
    // Also if there wasn't a string to begin with, return None

    if none_found || default_str.is_empty() {
        (None, Some(default_str))
    } else {
        let (new_term, rest_of_str) = next_term(cloned_str.clone());
        terms.push(new_term);
        assert!(terms.len() == n, "Wrong terms size");
        (Some(terms), rest_of_str)
    }

}
    
// Returns the next term of a given string
// If it is a char, return the char
// If it is an open paren, return the string between the first paren and the last
// If there are no more terms, return none
// Otherwise, panic
pub fn next_term(s: String) -> (String, Option<String>) {
    //dbg!(&s);
    let first_char = s.chars().next();
    match first_char {
        Some(x) => match x {
            '(' => here_to_next_paren(s),
            ')' => panic!("Extra close paren"),
           
            // Note that this allows too many characters, like [ to be used
            'A'..='z' => (first_char.unwrap().to_string(), { 
                let stripped = s.strip_prefix(first_char.unwrap()).unwrap();
                if stripped.is_empty() {
                    None
                } else {
                    Some(stripped.to_string())
                }
            }),
            _  => panic!("Invalid character"),
        }
        None => ("".to_string(), None),
    }
}


fn here_to_next_paren(s: String) -> (String, Option<String>) {
    let mut stack: usize = 1; // open - close -- greater than 0 or panic
    let mut in_parens = "".to_string();
    let mut out_of_parens = "".to_string();
    let mut found = false;

    // Skip first character
    for c in s.chars().skip(1) {
        if stack > 0 && !found {
            match c {
                '(' => { stack = stack + 1; in_parens.push(c) },
                ')' => { stack = stack - 1; in_parens.push(c) },
                'A'..='Z' => { in_parens.push(c); },
                'a'..='z' => { in_parens.push(c); },
                _ => panic!("Invalid character in next_paren"),
            }
        } else {
            found = true;
            out_of_parens.push(c);
        }
    };
    in_parens = in_parens.strip_suffix(')').expect("Unmatched paren").to_string();
    (in_parens, if found { Some(out_of_parens) } else { None } )
}
