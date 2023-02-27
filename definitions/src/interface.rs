#[repr(C)]
pub struct Interface {
    m_callbacks: Vec<fn(&str) -> bool>,
}

pub trait IInterface {
    fn add_callback(&mut self, callback: fn(&str) -> bool);
    fn remove_callback(&mut self, callback: fn(&str) -> bool);
    fn get_callbacks(&self) -> &Vec<fn(&str) -> bool>;
}

impl IInterface for Interface {
    fn add_callback(&mut self, callback: fn(&str) -> bool) {
        self.m_callbacks.push(callback);
    }

    fn remove_callback(&mut self, callback: fn(&str) -> bool) {
        self.m_callbacks
            .retain(|c| !std::ptr::eq(*c as *const (), callback as *const ()));
    }

    fn get_callbacks(&self) -> &Vec<fn(&str) -> bool> {
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
