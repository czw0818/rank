static mut COMPOSITETYPENUM:usize = 0;

#[derive(Debug,PartialEq)]
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
fn alloc_composite_type_id() -> usize{
    // 安全的,因为我们没有多线程,不会出现竞争.类似于C的i++
    unsafe{
        let alloc = COMPOSITETYPENUM;
        COMPOSITETYPENUM+=1;
        alloc
    }
}

impl Types{
    pub fn composite() -> Self{
        Self::CompositeType(alloc_composite_type_id())
    }
}

impl From<String> for Types{
    fn from(f: String) -> Self {
        match f.as_bytes(){
            b"int" => Self::Int,
            b"float" => Self::Float,
            b"uint" => Self::Uint,
            b"unit" => Self::Unit,
            b"str" => Self::Str,
            _ => unimplemented!()
        }
    }
}