use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Example {
    pub m_name: String,
    pub m_age: u8
}

// the trait is implemented in the application
pub trait IExample {
    fn print(&self);
}

impl Default for Example {
    fn default() -> Self {
        Self {
            m_name: String::new(),
            m_age: 0
        }
    }
}

impl Deref for dyn IExample {
    type Target = Example;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const dyn IExample as *const Example) }
    }
}

impl DerefMut for dyn IExample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut dyn IExample as *mut Example) }
    }
}