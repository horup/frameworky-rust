use legion::World;

pub trait SimpleSystem
{
    fn init(&mut self)
    {
        // do nothing is the default
    }

    fn execute(&mut self, _world:&mut World)
    {
        // do nothing is the default
    }

}