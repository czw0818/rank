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

/// 如果遇到自定义类型,那就使用一个新的数字来做Typeid
/// 为了保证全局统一，不出现两个类型使用同一个ID的情况,请从此函数获取ID
pub fn alloc_composite_type_id() -> usize{
    unsafe{
        let alloc = COMPOSITETYPENUM;
        COMPOSITETYPENUM+=1;
        alloc
    }
}