use std::{any::Any, cell::RefCell, rc::Rc};
use kiss3d::window::Window;

use crate::SimpleSystem;

#[derive(Default)]
pub struct Kiss3DSystem
{
    pub window:Option<Rc<RefCell<Window>>>
}

impl SimpleSystem for Kiss3DSystem
{
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }
}

impl Kiss3DSystem
{
    
}




