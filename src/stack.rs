pub struct Stack{
    data: [u16; 16],
    top: usize,
}

impl Stack {
    pub fn new()-> Self{
        Stack{
            data: [0;16],
            top:0
        }
    }
    pub fn push(&mut self, value: u16) -> Result<(),&str>{
        if self.top<16{
            self.data[self.top] = value;
            self.top+=1;
            Ok(())
        } else {
            Err("Stack overflow")
        }

    }
    pub fn pop(&mut self) -> Result<u16, &str>{
        if self.top>0{
            self.top-=1;
            Ok(self.data[self.top])
        }else{
            Err("Stack underflow")
        }
    }


}