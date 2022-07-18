use crate::frontend::tokens::*;
use crate::frontend::composite_type::*;
use crate::r#type::Types;

pub struct Parser<'a>(TokenStream<'a>);

macro_rules! debug {
    ($prog:expr) => {
        #[cfg(debug_assertions)]
            {
                println!($prog);
            }
    };
}
#[allow(dead_code)]
impl<'a> Parser<'a>{
    pub fn new(from:TokenStream<'a>) -> Self{
        Self(from)
    }
    fn get_type(&mut self) -> Option<Types>{
        let t = self.0.next()?;
        if t.is_keyword(){
            debug!("type can't be a keyword");
            return None;
        }
        if matches!(t,Token::Ident){
            return Some(From::from(self.0.slice()))
        }
        match t {
            Token::Unit => Some(Types::Unit),
            Token::LF => { // array
                let t = self.get_type()?;
                match self.0.next()?{
                    Token::RH => return Some(Types::Slice(Box::new(t))),
                    Token::Semicolon => {
                        debug_assert_eq!(self.0.next()?,Token::Ident);
                        let len = self.0.slice().parse().unwrap();
                        return Some(
                            Types::Array(Box::new(
                                (t,len)
                            ))
                        )
                    }
                    _ => return None
                }
            }
            _ => unimplemented!()
        }
    }
    /*
    fn peek(&mut self) -> Option<&Token>{
        match self.1 {
            Some(ref t) => Some(t),
            None =>{
                self.1 = self.next();
                self.1.as_ref()
            }
        }
    }*/
    fn next(&mut self) -> Option<Token>{
        self.0.next()
    }/*
    fn next_eq(&mut self,_n:Token) -> bool{
        match self.next(){
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
        if matches!(self.1.take(),Some(Token::Error)){
            panic!("frontend:parse.rs:Parser:Token::Error")
        }
    }
    fn get_ident(&mut self) -> Option<String>{
        if !self.peek_eq(Token::Ident){return None};
        Some(
            self.0.slice().to_string()
        )
    }*/
    pub fn try_struct(&mut self) -> Option<StructDeclare>{
        if self.next()? != Token::Struct{
            debug!("expect Struct");
            return None;
        }
        if self.next()? != Token::Ident{
            debug!("expect ident");
            return None;
        }
        let name = self.0.slice();
        if self.next()? != Token::LH{
            debug!("expect LH");
            return None;
        }
        let mut field = Vec::new();
        while self.next()? == Token::Ident {
            let f = self.0.slice().to_string();
            if self.0.next()? != Token::Colon{
                debug!("expect Colon(:)");
                return None;
            }
            let t = self.get_type()?;
            field.push((f,t))  
        }
        Some(
            StructDeclare{
                name:name.to_string(),field:field
            }
        )
    }
    fn try_union(&mut self) -> Option<UnionDeclare>{
        if self.next()? !=Token::Union{
            return None;
        }
        if self.next()? != Token::Ident{
            return None;
        }
        let name = self.0.slice();
        if self.next()? != Token::LH{
            return None;
        }
        let mut fields = Vec::new();    
        while self.next()? == Token::Ident {
            let f = self.0.slice().to_string();
            if self.next()? != Token::LX{
                return None;
            }
            let t = self.get_type()?;
            if self.next()? != Token::RX{
                return None;
            }
            fields.push((f,t))
        }
        Some(
            UnionDeclare{
                name:name.to_string(),field:fields
            }
        )
    }
    fn try_enum(&mut self) -> Option<EnumDeclare<u8>>{
        if self.next()?!=Token::Enum{
            return None;
        }
        if self.next()? != Token::Ident{
            return None;
        }
        let name = self.0.slice().to_string();
        if self.next()? != Token::LH{
            return None;
        }
        let mut fields = Vec::new();    
        while self.next()? == Token::Ident {
            let f = self.0.slice().to_string();
            if self.next()? != Token::LX{
                return None;
            }
            let t = self.get_type()?;
            if self.next()? != Token::RX{
                return None;
            }
            fields.push((f,t))
        }
        Some(
            EnumDeclare{
                union:UnionDeclare{
                    name:name,field:fields
                },
                tag:0u8
            }
        )
    }
}

#[test]
fn parse_struct(){
    use logos::Logos;
    const SOURCE:&str = r#"struct name{field1:uint field2:int}"#;
    let mut parser=Parser::new(Token::lexer(SOURCE));
    println!("{:?}",parser.try_struct().unwrap());
}