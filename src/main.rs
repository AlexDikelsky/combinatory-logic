fn main() {
    let s1 = "S((SS)X)YZ".to_string();
    dbg!(next_n_terms(4, s1));
}

#[test]
fn f() {
    let s1 = "S((SS)X)YZ".to_string();
    let a = next_term(s1);
    let b = next_term(a.clone().1.unwrap());
    let c = next_term(b.clone().1.unwrap());
   
    dbg!(a, b, c);
}


enum Op {
    S,
    K,
    I,
    Undef,
}



// If there are n more terms, return a vector of those terms
// Otherwise, return the original string, and None for the vector
//
// If there are excess characters, return a string in the second tuple place
fn next_n_terms(n: usize, s: String) -> (Option<Vec<String>>, Option<String>) {
    let mut v = vec![];
    let mut none_found = false;
    let mut i = 0;
    let default_str = s.clone();
    let mut cloned_str = s.clone();

    while i < n && !none_found {
        let a = next_term(cloned_str.clone());
        v.push(a.0);

        match a.1 {
            Some(x) => { cloned_str = x; },
            None => { none_found = true; },
        }; 
        i += 1;
    }

    match none_found {
        true  => (None, Some(default_str)),
        false => (Some(v), Some(cloned_str)),
    }

}
    
// Returns the next term of a given string
// If it is a char, return the char
// If it is an open paren, return the string between the first paren and the last
// If there are no more terms, return none
// Otherwise, panic
fn next_term(s: String) -> (String, Option<String>) {
    match s.chars().next() {
        Some(x) => match x {
            '(' => here_to_next_paren(s),
            ')' => panic!("Extra close paren"),
           
            // Note that this allows too many characters, like [ to be used
            'A'..='z' => (s.chars().next().unwrap().to_string(),
                                 Some(s.chars().skip(1).collect())),
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
                'A'..='z' => { in_parens.push(c); },
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


