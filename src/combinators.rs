use crate::interprate::next_n_terms;

// Returns the rest of the str applied to S
//    If there are enough args, applies
//    Else, returns string including the original S
#[allow(non_snake_case)]
pub fn S(s: String) -> String {
    let next_terms = next_n_terms(3, s);

    dbg!(&next_terms);

    match next_terms.0 {
        Some(v) => {
            assert!(v.len() == 3, "Somehow input to S was not three");
            format!("{x}{z}({y}{z}){r}", x=v[0], y=v[1], z=v[2], r=next_terms.1.unwrap_or_default())
        },
        None => {
            "S".to_string() + &next_terms.1.unwrap_or_default()
        }
    }
}


#[allow(non_snake_case)]
pub fn K(s: String) -> String {
    let next_terms = next_n_terms(2, s);
    dbg!(&next_terms);

    match next_terms.0 {
        Some(v) => {
            assert!(v.len() == 2, "Somehow input to K was not two");
            format!("{x}{r}", x=v[0], r=next_terms.1.unwrap_or_default())
        },
        None => {
            "K".to_string() + &next_terms.1.unwrap_or_default()
        }
    }
}

#[allow(non_snake_case)]
pub fn I(s: String) -> String {
    let next_terms = next_n_terms(1, s);
    dbg!(&next_terms);

    match next_terms.0 {
        Some(v) => {
            assert!(v.len() == 1, "Somehow input to I was not one");
            format!("{x}{r}", x=v[0], r=next_terms.1.unwrap_or_default())
        },
        None => {
            "I".to_string() + &next_terms.1.unwrap_or_default()
        }
    }
}
