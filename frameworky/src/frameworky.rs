use legion::*;
use crate::SimpleSystem;

#[derive(Default)]
pub struct Frameworky
{
    world:World,
    systems:Vec<Box<dyn SimpleSystem>>
}

impl Frameworky
{
    pub fn push_system<T:SimpleSystem + 'static>(&mut self, s:T)
    {
        self.systems.push(Box::new(s));
    }

    pub fn run(&mut self) -> !
    {
        for s in self.systems.iter_mut()
        {
            s.init();
        }

        loop {
            for s in self.systems.iter_mut()
            {
                s.execute(&mut self.world);
            }
        }
    }
}