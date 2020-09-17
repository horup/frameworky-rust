use std::rc::Rc;
use crate::{Context, Event};

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

    fn execute(&mut self, _context:&mut Context, _event:&Rc<dyn Event>)
    {
        // do nothing is the default
    }

}