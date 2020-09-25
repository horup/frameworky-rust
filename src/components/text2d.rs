pub struct Text2D
{
    pub text:String,
    pub x:i32,
    pub y:i32,
    pub size:i32    
}

impl Text2D
{
    pub fn new(text:String, x:i32, y:i32, size:i32) -> Self
    {
        Text2D {
            text:text,
            x:x,
            y:y,
            size:size
        }
    }
}