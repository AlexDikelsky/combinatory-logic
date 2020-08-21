use std::fs;

use crate::combinators::{ S, K, I };
use crate::run::eval_sub_ctx;


// There's a file in the root with a ton of programs
// and the normal form of the program. Read the
// file and assert that the normal form in the file
// is the same as the normal form here
#[test]
fn do_all() {

    // Probably won't work on other people's computers
    let file_contents = fs::read_to_string(
        "/home/adikelsky/math/programming-languages/combinatory-logic/src/tests/normal_forms.txt"
        ).unwrap();
    for line in file_contents.lines() {
        let splited: Vec<&str> = line.split(':').collect();
        assert!(splited.len() == 2);

        let expected = splited[1].to_string();
        let mine = eval_sub_ctx(splited[0].to_string());


        if expected != mine {
            println!("{}\n  me: {}\n  co: {}", splited[0], expected, mine);
        }


        assert_eq!(expected, mine);
    }

}

