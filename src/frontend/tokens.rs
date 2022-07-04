use logos::Logos;
use Token::*;

#[derive(Logos,Debug,PartialEq)]
enum Token{
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("r#")]
    SpeIde,
    #[token("#")]
    Well,
    #[regex("[a-zA-Z]+")]
    Ident,
    #[token("type")]
    Type,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("struct")]
    Struct,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("==")]
    IsEqual,
    #[token("!=")]
    NotEqual,
    #[token("=")]
    EqualSign,
    #[token("(")]
    LX,
    #[token(")")]
    RX,
    #[token("[")]
    LF,
    #[token("]")]
    RF,
    #[token("{")]
    LH,
    #[token("}")]
    RH,
    #[token("panic")]
    Panic,
    #[error]
    Error,
}
impl Token{
    pub fn is_keyword(&self) -> bool{
        matches!(self,If | Else | For | In | While | Type | Struct | Panic) 
    }
}