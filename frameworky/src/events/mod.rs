#[derive(Debug, Default)]
pub struct MouseButtonDown {
    pub screen_x:f64,
    pub screen_y:f64,
    pub button:u32
}

#[derive(Debug, Default)]
pub struct MouseButtonUp {
    pub screen_x:f64,
    pub screen_y:f64,
    pub button:u32
}

#[derive(Debug, Default)]
pub struct KeyEvent {
    pub key:u32,
    pub down:bool
}