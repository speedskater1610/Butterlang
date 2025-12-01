mod lexer;
mod parser;

fn main() {
    println!("{:?}", parser::parse_tokens(lexer::lex("struct person {
        name = '',
        age = 0,
    }

    let a = person { name='liam', age=20 };

    print(a.name);
    ")));
}
