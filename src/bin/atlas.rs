//use std::path::Path;

use atlas::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.parse(String::from(r#"(print (concat "Result: " (* 2 (+ 2 1))))"#));
    //let path = Path::new("/home/steew/Projects/local/atlas/src/bin/tests/test.atl");
    //parser.read_file(path);
    // TODO: fix, redundant passing of the String, ugly clone
    //parser.parse(parser.contents.clone());
}
