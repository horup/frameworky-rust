use std::{rc::Rc, any::Any};
use legion::world::World;

pub struct Context
{
    pub running:bool,
    pub world:World,
    pub events:Vec<Rc<dyn Any>>
}

impl Default for Context
{
    fn default() -> Self
    {
        Context { running:true, world:World::default(), events:Vec::new() }
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