use std::{rc::Rc, any::Any};
use legion::world::World;

pub struct Time
{
    pub t:f64,
    pub dt:f64,
    pub current:f64,
    pub accumulator:f64
}

impl Default for Time
{
    fn default() -> Self {
        let dt = 1.0 / 20.0; 
        Time {
            t:0.0,
            dt:dt,
            current:instant::now() / 1000.0,
            accumulator:0.0
        }
    }
}


pub struct Context
{
    pub once:bool,
    pub world:World,
    pub events:Vec<Rc<dyn Any>>,
    pub time:Time
}

impl Default for Context
{
    fn default() -> Self
    {
        Context { once:false, world:World::default(), events:Vec::new(), time:Time::default() }
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