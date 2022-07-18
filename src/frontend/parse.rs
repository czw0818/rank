use crate::frontend::tokens::*;
use crate::frontend::composite_type::*;
use crate::r#type::Types;

pub struct Parser<'a>(TokenStream<'a>,Option<Token>);

macro_rules! assert_next {
    ($s:expr,$t:expr) => {
        if !(Self::next_eq($s,$t)){
            return None;
        }
    };
}
#[allow(dead_code)]
impl<'a> Parser<'a>{
    fn new(from:TokenStream<'a>) -> Self{
        Self(from,None)
    }
    fn peek(&mut self) -> Option<&Token>{
        match self.1 {
            Some(ref t) => Some(t),
            None =>{
                self.1 = self.0.next();
                self.1.as_ref()
            }
        }
    }
    fn next_eq(&mut self,_n:Token) -> bool{
        match self.0.next(){
            Some(t) => matches!(t,_n),
            None => false
        }
    }
    fn peek_eq(&mut self,_n:Token) -> bool{
        match self.peek(){
            Some(t)  => matches!(t,_n),
            None => false
        }
    }
    fn clear_peek(&mut self){
        self.1 = None
    }
    fn get_ident(&mut self) -> Option<String>{
        if !self.peek_eq(Token::Ident){return None};
        Some(
            self.0.slice().to_string()
        )
    }
    fn try_struct(&mut self) -> Option<StructDeclare>{
        assert_next!(self,Token::Struct);
        let name = self.get_ident()?;
        self.clear_peek();
        assert_next!(self,Token::LH);
        let mut field = Vec::new();
        while self.peek_eq(Token::Ident) {
            let f = self.get_ident()?;
            assert_next!(self,Token::Colon);
            let t = self.get_ident()?;
            field.push((f,Types::from(t)))  
        }
        assert_next!(self,Token::RH);
        self.clear_peek();
        Some(
            StructDeclare{
                name:name,field:field
            }
        )
    }
}