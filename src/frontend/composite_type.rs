use crate::r#type::Types;

pub struct StructDeclare{
    pub name:String,
    pub field:Vec<(String,Types)>
}

pub struct UnionDeclare(Vec<(String,Types)>);

pub struct EnumDeclare(UnionDeclare,usize);