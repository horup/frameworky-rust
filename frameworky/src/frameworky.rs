use crate::{SimpleSystem, Context, Event};

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

            if !self.context.running {
                return;
            }
        }
    }

    pub fn execute(&mut self, context:&mut Context, event:&dyn Event)
    {
        for s in self.systems.iter_mut() {
            s.execute(context, event);
        }
    }
}