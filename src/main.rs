mod aux;
mod parse;
mod term;

use crate::parse::parse;
use crate::term::Term;

fn main() {
    let v = "ABC".chars().collect();
    parse(v, Term::Nil);
}
    
