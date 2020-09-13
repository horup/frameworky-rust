use std::iter::Flatten;

use crate::components::Body;
use crate::{legion::*, components::Transform};
use query::{View, Query, DefaultFilter, ChunkIter};
use legion::{World};


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


impl Context
{
}