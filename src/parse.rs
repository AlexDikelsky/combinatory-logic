use crate::aux::apply;
use crate::term::Term;
use crate::term::Term::{ Variable, Application, Combinator, Nil };



// Takes string of combinators/variables, and produces 
// applications and vars off of it
pub fn parse(s: Vec<char>, loc: Term) -> Term {

    if s == vec![] {
        Nil
    } else {
        match s[0] {
            '(' => apply(loc.clone(), parse(tail, loc)),
            ')' => Nil,
            'a'..='z' => apply(Variable(s[0]), parse(tail, loc)),
            'A'..='Z' => apply(Combinator(s[0]), parse(tail, loc)),
            _ => panic!("Illegal character"),
        },
    }
}
