use crate::IInterface;

#[repr(C)]
pub struct Plugin {
    pub m_name: &'static str,
    pub m_authors: &'static [&'static str],
    pub m_init: Option<fn(&mut dyn IInterface)>,
    pub m_shutdown: Option<fn(&mut dyn IInterface)>,
}

#[macro_export]
macro_rules! declare_plugin {
    ($name:expr, $authors:expr, $init:expr, $shutdown:expr) => {
        #[no_mangle]
        pub fn dll_info() -> &'static $crate::Plugin {
            static PLUGIN_INFO: $crate::Plugin = $crate::Plugin {
                m_name: $name,
                m_authors: $authors,
                m_init: $init,
                m_shutdown: $shutdown,
            };
            &PLUGIN_INFO
        }
    };
}
