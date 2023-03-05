#[repr(C)]
#[derive(Default)]
pub struct Example {
    pub m_name: String,
    pub m_age: u8,
}

// the trait is implemented in the application
pub trait IExample {
    fn get_ref(&self) -> &Example;
    fn get_mut(&mut self) -> &mut Example;
    fn print(&self);
}
