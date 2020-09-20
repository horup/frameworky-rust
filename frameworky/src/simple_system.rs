use std::{any::Any};
use kiss3d::window::Window;

use crate::{Context};

pub trait SimpleSystem
{
    fn once(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn update(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn execute(&mut self, _context:&mut Context, _event:&dyn Any)
    {
        // do nothing is the default
    }

    fn update_with_window(&mut self, _context:&mut Context, window:&mut Window)
    {
        // do nothing is the default
    }

}