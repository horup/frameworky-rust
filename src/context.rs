use std::{rc::Rc, any::Any};
use kiss3d::window::Window;
use legion::world::World;

pub struct Context
{
    pub once:bool,
    pub world:World,
    pub events:Vec<Rc<dyn Any>>
}

impl Default for Context
{
    fn default() -> Self
    {
        Context { once:false, world:World::default(), events:Vec::new() }
    }
}


impl Context
{
    pub fn push_event<T:Sized + Any + 'static>(&mut self, event:T)
    {
        let rc:Rc<dyn Any> = Rc::new(event);
        self.events.push(rc);
    }
}