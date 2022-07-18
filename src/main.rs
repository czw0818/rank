pub mod frontend;
pub mod r#type;
pub mod rt;

fn main() {}
#[test]
fn parse_struct(){
    use crate::frontend::{parse::Parser,tokens::Token};
    use logos::Logos;
    const SOURCE:&str = r#"struct name{field1:uint field2:int}"#;
    let mut parser=Parser::new(Token::lexer(SOURCE));
    println!("{:?}",parser.try_struct().unwrap());
}