use logos::Logos;
use Token::*;

#[derive(Logos,Debug,PartialEq)]
pub enum Token{
    #[regex(r"[0-9]+")]
    Num,
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
    #[regex("[a-zA-Z][a-zA-Z0-9]+")]
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
    #[token("union")]
    Union,
    #[token("Enum")]
    Enum,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("()")]
    Unit,
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
        matches!(self,If | Else | For | In | While | Type | Struct | Panic|Enum|Union) 
    }
}
pub type TokenStream<'a> = logos::Lexer<'a,Token>;