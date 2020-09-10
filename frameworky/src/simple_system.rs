use legion::World;

use crate::Context;

pub trait SimpleSystem
{
    fn once(&mut self, context:&mut Context)
    {
        // do nothing is the default
    }

    fn update(&mut self, context:&mut Context)
    {
        // do nothing is the default
    }

}