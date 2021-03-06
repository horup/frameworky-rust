use std::{any::Any};
use crate::{Context};

pub trait SimpleSystem : Any
{
    fn once(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn update(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn before_fixed_update(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn fixed_update(&mut self, _context:&mut Context)
    {
        // do nothing is the default
    }

    fn execute(&mut self, _context:&mut Context, _event:&dyn Any)
    {
        // do nothing is the default
    }

    fn as_any(&self) -> Option<&dyn Any>
    {
        None
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any>
    {
        None
    }
}