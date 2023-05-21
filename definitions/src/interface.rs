#[repr(C)]
#[derive(Default)]
pub struct Interface {}

pub trait IInterface {}

impl IInterface for Interface {}
