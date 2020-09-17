use std::rc::Rc;
use crate::event::Event;
use legion::world::World;

pub struct Context
{
    pub running:bool,
    pub world:World,
    pub events:Vec<Rc<dyn Event>>
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
    pub fn push_event<T:Sized + Event + 'static>(&mut self, event:T)
    {
        let rc:Rc<dyn Event> = Rc::new(event);
        self.events.push(rc);
    }
}