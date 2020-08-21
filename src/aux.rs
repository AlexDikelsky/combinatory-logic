use crate::term::Term;

pub fn apply(a: Term, b: Term) -> Term {
    Term::Application(Box::new(a), Box::new(b))
}
