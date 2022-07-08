use crate::r#type::Types;

pub struct CallFunctionHandle{
    pub name:String,
    pub args:Vec<Expr>
}

pub struct LetHandle{
    pub name:String,
    pub value:Expr
}

pub struct WhileHandle{
    pub cond:Expr,
    pub state:Vec<State>,  // while 体的语句块
}

pub struct DeclareStruct{
    pub r#pub:bool,
    pub fields:Vec<(String,Types)>
}

pub struct DeclareEnum{
    pub r#pub:bool,
    pub fields:Vec<(String,Types)>
}

pub struct DeclareUnion{
    pub r#pub:bool,
    pub fields:Vec<(String,Types)>
}

pub enum DeclareNewType{
    Struct(Box<DeclareStruct>),
    Enum(Box<DeclareEnum>),
    Union(Box<DeclareUnion>)
}
pub enum Expr{
    Call(Box<CallFunctionHandle>),
    While(Box<WhileHandle>),
    Break(Box<Expr>)
}

pub enum State{
    Expr(Expr),
    Let(Box<LetHandle>),
    Declare(DeclareNewType)
}