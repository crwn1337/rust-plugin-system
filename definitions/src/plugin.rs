use crate::IInterface;

#[repr(C)]
pub struct Plugin {
    pub m_name: &'static str,
    pub m_authors: &'static [&'static str],
    pub m_init: Option<fn(&mut dyn IInterface)>,
    pub m_shutdown: Option<fn(&mut dyn IInterface)>,
    pub m_state: Box<dyn IPlugin>,
}

pub trait IPlugin {
    fn callback(&mut self, example: &mut dyn crate::IExample) -> bool;
}

// CALL THIS ONLY ONCE!
#[macro_export]
macro_rules! plugin {
    ($name:expr, $authors:expr, $init:expr, $shutdown:expr, $state:expr) => {
        #[no_mangle]
        pub fn __dll_info() -> crate::Plugin {
            crate::Plugin {
                m_name: $name,
                m_authors: $authors,
                m_init: $init,
                m_shutdown: $shutdown,
                m_state: Box::new($state),
            }
        }
    };
}
