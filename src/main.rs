mod parser;
mod util;

fn main()
{
    println!("{:?}", parser::lex::lex(b"hello\n23 "));
}