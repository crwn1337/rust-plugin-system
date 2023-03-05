use std::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Default)]
pub struct Example {
    pub m_name: String,
    pub m_age: u8,
}

// the trait is implemented in the application
pub trait IExample {
    fn print(&self);
}

impl Deref for dyn IExample + '_ {
    type Target = Example;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const dyn IExample as *const Example) }
    }
}

impl DerefMut for dyn IExample + '_ {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut dyn IExample as *mut Example) }
    }
}
