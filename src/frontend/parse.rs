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
    pub fn new(from:TokenStream<'a>) -> Self{
        Self(from,None)
    }
    fn get_type(&mut self) -> Option<Types>{
        let first =  self.next()?;
        if first.is_keyword(){return None;}
        if let Some(n)=self.get_ident(){
            return Some(Types::from(n))
        }
        match first {
            Token::Unit => Some(Types::Unit),
            Token::LF => { // array
                let t = self.get_type()?;
                assert_next!(self,Token::Semicolon);
                let len=self.get_ident().unwrap().parse().unwrap();
                Some(Types::Array(Box::new((t,len))))
            }
            _ => unimplemented!()
        }
    }
    fn peek(&mut self) -> Option<&Token>{
        match self.1 {
            Some(ref t) => Some(t),
            None =>{
                self.1 = self.next();
                self.1.as_ref()
            }
        }
    }
    fn next(&mut self) -> Option<Token>{
        match self.1.take(){
            Some(t) => Some(t),
            None => self.next()
        }
    }
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
        self.1.take();
    }
    fn get_ident(&mut self) -> Option<String>{
        if !self.peek_eq(Token::Ident){return None};
        Some(
            self.0.slice().to_string()
        )
    }
    pub fn try_struct(&mut self) -> Option<StructDeclare>{
        assert_next!(self,Token::Struct);
        let name = self.get_ident()?;
        self.clear_peek();
        assert_next!(self,Token::LH);
        let mut field = Vec::new();
        while self.peek_eq(Token::Ident) {
            let f = self.get_ident()?;
            assert_next!(self,Token::Colon);
            let t = self.get_type()?;
            field.push((f,t))  
        }
        assert_next!(self,Token::RH);
        self.clear_peek();
        Some(
            StructDeclare{
                name:name,field:field
            }
        )
    }
    fn try_union(&mut self) -> Option<UnionDeclare>{
        assert_next!(self,Token::Union);
        let name = self.get_ident()?;
        assert_next!(self,Token::LH);
        let mut fields = Vec::new();    
        while self.peek_eq(Token::Ident) {
            let f = self.get_ident()?;
            assert_next!(self,Token::LX);
            let t = self.get_type()?;
            assert_next!(self,Token::RX);
            fields.push((f,t))
        }
        assert_next!(self,Token::RH);
        self.clear_peek();
        Some(
            UnionDeclare{
                name:name,field:fields
            }
        )
    }
    fn try_enum(&mut self) -> Option<EnumDeclare<u8>>{
        assert_next!(self,Token::Enum);
        let name = self.get_ident()?;
        assert_next!(self,Token::LH);
        let mut fields = Vec::new();    
        while self.peek_eq(Token::Ident) {
            let f = self.get_ident()?;
            assert_next!(self,Token::LX);
            let t = self.get_type()?;
            assert_next!(self,Token::RX);
            fields.push((f,t))
        }
        assert_next!(self,Token::RH);
        self.clear_peek();
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