
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Shape 
{
    Sphere = 0,
    Plane = 1
}

#[derive(Debug, Copy, Clone)]
pub struct Body
{
    pub body_handle:Option<generational_arena::Index>,
    pub shape:Shape
}

impl Body 
{
    pub fn new(shape:Shape) -> Self 
    {
        Self
        {
            shape:shape,
            body_handle:None
        }
    }
}

impl Default for Body
{
    fn default() -> Self 
    {
        Self
        {
            shape:Shape::Sphere, 
            body_handle:None
        }
    }
}