use std::{env, fs};

use while_lang::{Interpreter, Lexer, Parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).expect("missing path to input file");
    let input = fs::read_to_string(path).expect("failed to read input file");
    let tokens = Lexer::new(&input).lex();
    let tree = Parser::new(tokens).parse();
    let result = Interpreter::new(
        &args
            .get(2..)
            .map(|args| {
                args.iter()
                    .map(|num| {
                        num.parse()
                            .expect("start values for variables should be valid u64 integers")
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::new),
    )
    .run(tree);
    println!("{result}");
}
