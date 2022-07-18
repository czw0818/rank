pub struct Frame{
    pub parent : *mut Frame,
    pub line   : usize,
    pub scope  : hashbrown::HashMap<String,usize>,
    pub name   : String,
}

impl Frame{
    pub fn parent(&self) -> Option<&Self>{
        if self.parent.is_null(){
            None // 第一个函数
        }else{
            unsafe{
                Some(&*self.parent)
            }
        }
    }
    pub fn print_stack(&self) -> String{
        todo!()
    }
}