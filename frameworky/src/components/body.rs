
#[derive(Debug, Default, Copy, Clone)]
pub struct Body
{
    pub body_handle:Option<generational_arena::Index>    
}