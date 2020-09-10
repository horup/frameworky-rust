use legion::World;


pub struct Context
{
    pub running:bool,
    pub world:World
}

impl Default for Context
{
    fn default() -> Self
    {
        Context { running:true, world:World::default() }
    }
}