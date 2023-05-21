// this is an example of an interface that you **might** want to implement
// so the init and shutdown functions can use this

#[repr(C)]
#[derive(Default)]
pub struct Interface {
    pub m_counter: u32,
}

pub trait IInterface {
    fn inc_counter(&mut self) -> u32;
    fn dec_counter(&mut self) -> u32;
}

impl IInterface for Interface {
    fn inc_counter(&mut self) -> u32 {
        self.m_counter += 1;
        self.m_counter
    }
    
    fn dec_counter(&mut self) -> u32 {
        self.m_counter -= 1;
        self.m_counter
    }
}
