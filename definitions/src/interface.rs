#[repr(C)]
pub struct Interface {
    m_callbacks: Vec<fn(str: String) -> bool>,
}

pub trait IInterface {
    fn add_callback(&mut self, callback: fn(str: String) -> bool);
    fn remove_callback(&mut self, callback: fn(str: String) -> bool);
    fn get_callbacks(&self) -> &Vec<fn(str: String) -> bool>;
}

impl IInterface for Interface {
    fn add_callback(&mut self, callback: fn(str: String) -> bool) {
        self.m_callbacks.push(callback);
    }

    fn remove_callback(&mut self, callback: fn(str: String) -> bool) {
        self.m_callbacks.retain(|x| x != &callback);
    }

    fn get_callbacks(&self) -> &Vec<fn(str: String) -> bool> {
        &self.m_callbacks
    }
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            m_callbacks: Vec::new(),
        }
    }
}
