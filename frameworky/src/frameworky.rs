use std::rc::Rc;

use crate::{SimpleSystem, Context};

#[derive(Default)]
pub struct Frameworky
{
    pub systems:Vec<Box<dyn SimpleSystem>>,
    pub context:Context
}

impl Frameworky
{
    pub fn push_system<T:SimpleSystem + 'static>(&mut self, s:T)
    {
        self.systems.push(Box::new(s));
    }

    pub fn run(&mut self)
    {
        for s in self.systems.iter_mut()
        {
            s.once(&mut self.context);
        }

        loop {
            for s in self.systems.iter_mut()
            {
                s.update(&mut self.context);
            }

            let events = self.context.events.clone();
            self.context.events.clear();

            for s in self.systems.iter_mut()
            {
                for e in events.iter() {
                    s.execute(&mut self.context, e.as_ref());
                }
            }

            if !self.context.running {
                return;
            }
        }
    }
}