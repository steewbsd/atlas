//use std::path::Path;
use std::env;

use atlas::Parser;

fn main() {
    let mut parser = Parser::new();
    let args: Vec<String> = env::args().collect();
    parser.parse(String::from(&args[1]));
    parser.reduce_all();

    //parser.parse(String::from(r#"(+ 1 (+ 1 1) (+ 2 2))"#));
    // parser.reduce_all();
    //let path = Path::new("/home/steew/Projects/local/atlas/src/bin/tests/test.atl");
    //parser.read_file(path);
    // TODO: fix, redundant passing of the String, ugly clone
    //parser.parse(parser.contents.clone());
}
