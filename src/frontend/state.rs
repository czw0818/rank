
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
    pub state:Vec<State>,
}
pub enum Expr{
    Call(Box<CallFunctionHandle>),
    While(Box<WhileHandle>),
    Break(Box<Expr>)
}

pub enum State{
    Expr(Expr),
    Let(Box<LetHandle>),
    Declare()
}