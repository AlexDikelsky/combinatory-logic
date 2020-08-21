use crate::Term::{ Application, Variable, Combinator, Nil };
use crate::aux::apply;
pub enum Term {

    Application(Box<Term>, Box<Term>),

    // Single char, lowercase
    Variable(char),

    // Single chars, uppercase
    Combinator(char),

    // Not all arguments need to be "taken", so this gives
    // you a way to have applications, but no argument yet
    Nil,
}

impl Clone for Term {
    fn clone(&self) -> Self {
        match &self {
            Application(a, b) => apply(*a.clone(), *b.clone()),
            Variable(c) => Variable(*c),
            Combinator(c) => Combinator(*c),
            Nil => Nil,
        }
    }
}
    
