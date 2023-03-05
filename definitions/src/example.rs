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

impl AsRef<Example> for dyn IExample + '_ {
    fn as_ref(&self) -> &Example {
        unsafe { &*(self as *const dyn IExample as *const Example) }
    }
}

impl AsMut<Example> for dyn IExample + '_ {
    fn as_mut(&mut self) -> &mut Example {
        unsafe { &mut *(self as *mut dyn IExample as *mut Example) }
    }
}
