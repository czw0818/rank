use super::tokens::Token;
use logos::Logos;

struct TokenStream<'a>{
    peek:Option<Token>,
    token_gen:logos::Lexer<'a,Token>
}

impl TokenStream<'_>{
    fn next(&self) -> Option<Token>{
        todo!()
        // self.peek.or_else(||self.token_gen.next())
    }
}