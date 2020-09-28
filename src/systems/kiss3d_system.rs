use std::{any::Any};
use kiss3d::window::Window;

use crate::{Context, SimpleSystem};

#[derive(Default)]
pub struct Kiss3DSystem
{
    pub window:Option<*mut Window>,
}

impl SimpleSystem for Kiss3DSystem
{
    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn update_fixed(&mut self, context:&mut Context) {
        if let Some(window_ptr) = self.window
        {
            // window is a pointer to the underlying owner, which is unsafe
            unsafe {
                let window = &mut (*window_ptr);
                self.update_fixed(context, window);
            }
        }
    }

    fn update(&mut self, context:&mut Context) {
        if let Some(window_ptr) = self.window
        {
             // window is a pointer to the underlying owner, which is unsafe
            unsafe {
                let window = &mut (*window_ptr);
                self.update(context, window);
            }
        }
    }
}

impl Kiss3DSystem
{
    pub fn update_fixed(&mut self, context:&mut Context, window:&mut Window)
    {

    }

    pub fn update(&mut self, context:&mut Context, window:&mut Window)
    {

    }


}




