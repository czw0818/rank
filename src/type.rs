static mut COMPOSITETYPENUM:usize = 0;

pub enum Types{
    Int,
    Uint,
    Unit,
    Float,
    Str,
    Array(Box<(Types,usize)>),
    Slice(Box<Types>),
    CompositeType(usize)
}

pub fn alloc_composite_type_id() -> usize{
    unsafe{
        let alloc = COMPOSITETYPENUM;
        COMPOSITETYPENUM+=1;
        alloc
    }
}