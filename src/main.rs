use std::fs;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    // let input = "متغير س = ٥ + ١٠.";
    let input = fs::read_to_string("./تجربة.عمود").unwrap();
    let tokens = lexer::run(&input);
    println!("{:#?}", tokens);
    let ast = parser::run(tokens);
    // println!("{:#?}", ast);
    interpreter::run(ast);
}
