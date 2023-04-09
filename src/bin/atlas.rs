use atlas::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.parse(String::from("(Hello (world (earth)) (lul))"));
}
