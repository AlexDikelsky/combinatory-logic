#[derive(Copy,Clone,Debug)]
pub enum Term {
    Application(Box<Term>, Box<Term>),
    Variable(char),
    Combinator(char),
}
