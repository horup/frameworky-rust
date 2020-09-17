use std::{any::Any};
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

}