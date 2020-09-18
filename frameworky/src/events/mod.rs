#[derive(Debug, Default)]
pub struct KeyEvent {
    pub key:u32,
    pub down:bool
}

#[derive(Debug)]
pub enum MouseEventType 
{
    ButtonDown,
    ButtonUp
}

#[derive(Debug)]
pub struct MouseEvent 
{
    pub screen_x:i32,
    pub screen_y:i32,
    pub button:u32,
    pub event_type:MouseEventType
}