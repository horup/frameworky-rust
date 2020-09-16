use legion::World;

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

    fn execute(&mut self, _context:&mut Context, event:&dyn Event)
    {
        // do nothing is the default
    }

}