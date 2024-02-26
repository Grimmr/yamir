mod parser;
mod util;

fn main()
{
    println!("{:?}", parser::parser::parse_from_bytes(b"dat w 1 25"));
}