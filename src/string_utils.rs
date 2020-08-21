// Given a string starting one place after the paren to delete,
// delete the paren that matches the previous parenthesis
pub fn delete_matching_paren(s: String) -> String {
    // Can be unsigned because of cases like )a)a where there 
    // is an extra paren at the end, but it doesn't matter for
    // the deletion of the first parenthesis
    let mut stack = 1isize;
    let mut found = false;

    let copied = s.chars().fold("".to_string(), |mut a, b| {
        let result = match b {
            '(' => { stack += 1; b },
            ')' => { stack -= 1; b }, 
            c => c
        };
        if stack == 0 && !found {
            found = true;
            a
        } else {
            a.push(result);
            a
        }
    });
    assert!(found == true, "Unmatched Parenthesis");
    copied
}


// Returns the chars after the first
// If s is empty, return none, else return the rest
//      If input was only 1 char, return empty str
pub fn tail(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.chars().skip(1).collect())
    }
}

// First character of str
pub fn head(s: &str) -> Option<char> {
    s.chars().nth(0)
}

// Returns the string between the current location and the 
// matching parenthesis
pub fn here_to_next_paren(s: String) -> (String, Option<String>) {
    let mut stack: usize = 1; // open - close -- greater than 0 or panic
    let mut in_parens = "".to_string();
    let mut out_of_parens = "".to_string();
    let mut found = false;

    // Skip first character
    for c in s.chars().skip(1) {
        if stack > 0 && !found {
            match c {
                '(' => { stack += 1; in_parens.push(c) },
                ')' => { stack -= 1; in_parens.push(c) },
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
