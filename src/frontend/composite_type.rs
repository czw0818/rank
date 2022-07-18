use crate::r#type::Types;

#[derive(Debug)]
pub struct StructDeclare{
    pub name:String,
    pub field:Vec<(String,Types)>
}

#[derive(Debug)]
pub struct UnionDeclare{
    pub name:String,
    pub field:Vec<(String,Types)>
}

#[derive(Debug)]
pub struct EnumDeclare<T:PartialOrd>{
    pub union:UnionDeclare,
    pub tag:T
}