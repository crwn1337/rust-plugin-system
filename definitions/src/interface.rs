#[repr(C)]
#[derive(Default)]
pub struct Interface {
    m_callbacks: Vec<fn(&dyn crate::IExample) -> bool>,
}

pub trait IInterface {
    fn add_callback(&mut self, callback: fn(&dyn crate::IExample) -> bool);
    fn remove_callback(&mut self, callback: fn(&dyn crate::IExample) -> bool);
    fn get_callbacks(&self) -> &Vec<fn(&dyn crate::IExample) -> bool>;
}

impl IInterface for Interface {
    fn add_callback(&mut self, callback: fn(&dyn crate::IExample) -> bool) {
        self.m_callbacks.push(callback);
    }

    fn remove_callback(&mut self, callback: fn(&dyn crate::IExample) -> bool) {
        self.m_callbacks
            .retain(|c| *c as *const () != callback as *const ());
    }

    fn get_callbacks(&self) -> &Vec<fn(&dyn crate::IExample) -> bool> {
        &self.m_callbacks
    }
}
