type Callback = fn(&mut dyn crate::IExample) -> bool;

#[repr(C)]
#[derive(Default)]
pub struct Interface {
    m_callbacks: Vec<Callback>,
}

pub trait IInterface {
    fn add_callback(&mut self, callback: Callback);
    fn remove_callback(&mut self, callback: Callback);
    fn get_callbacks(&self) -> &[Callback];
}

impl IInterface for Interface {
    fn add_callback(&mut self, callback: Callback) {
        self.m_callbacks.push(callback);
    }

    fn remove_callback(&mut self, callback: Callback) {
        self.m_callbacks
            .retain(|c| *c as *const () != callback as *const ());
    }

    fn get_callbacks(&self) -> &[Callback] {
        &self.m_callbacks
    }
}
