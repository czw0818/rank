use std::{iter::Peekable, ops::Deref};

use super::tokens::Token;
use logos::{Logos,Lexer};

pub(in crate) struct TokenStream<'a>{
    token_gen:Peekable<Lexer<'a,Token>>
}

impl<'a> TokenStream<'a>{
    pub(in crate::frontend) fn new(arg:Lexer<'a,Token>) -> Self{
        Self{
            token_gen:arg.into_iter().peekable()
        }
    }
}

impl<'a> Deref for TokenStream<'a>{
    type Target = Peekable<Lexer<'a,Token>>;
    fn deref(&self) -> &Self::Target {
        &self.token_gen
    }
}

enum AnalyzerStatuations{
    ZeroSizeType,
    Struct,
    Enum,
    Union
}
pub struct Analyzer<'a>{
    toks:TokenStream<'a>,
    construct:AnalyzerStatuations
}

impl Analyzer<'_>{
    fn the_next(&mut self,expect:Token) -> bool{
        self.toks.next().expect("unexpect EOF") == expect
    }
    fn try_construct_struct(&mut self) -> Option<crate::frontend::composite_type::StructDeclare>{
        if !self.the_next(Token::Struct){
            return None
        }
        todo!()
    }
}