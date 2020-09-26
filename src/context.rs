use std::{rc::Rc, any::Any};
use legion::world::{Duplicate, World};

use crate::components::{Body, Transform};

pub struct Time
{
    pub t:f64,
    pub dt:f64,
    pub current:f64,
    pub accumulator:f64,
    pub alpha:f64
}

impl Default for Time
{
    fn default() -> Self {
        let dt = 1.0 / 20.0; 
        Time {
            t:0.0,
            dt:dt,
            current:instant::now() / 1000.0,
            accumulator:0.0,
            alpha:0.0
        }
    }
}


pub struct Context
{
    pub once:bool,
    pub world:World,
    pub events:Vec<Rc<dyn Any>>,
    pub time:Time,
    pub fixed_update_count:u64,
}

impl Default for Context
{
    fn default() -> Self
    {
        Context { once:false, world:World::default(), events:Vec::new(), time:Time::default(), fixed_update_count:0 }
    }
}


impl Context
{
    pub fn clone_world(&self) -> World
    {
        let mut cloned:World = World::default();
        let mut merger = Duplicate::default();
        merger.register_copy::<Transform>();
        merger.register_copy::<Body>();
        cloned.clone_from(&self.world, &legion::any(), &mut merger);
        return cloned;
    }

    pub fn push_event<T:Sized + Any + 'static>(&mut self, event:T)
    {
        let rc:Rc<dyn Any> = Rc::new(event);
        self.events.push(rc);
    }
}