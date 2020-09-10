use kiss3d::window::Window;

use crate::{SimpleSystem, Context};

pub struct Kiss3DSystem
{
    window:Window
}

impl Kiss3DSystem
{
    pub fn new(title:&str)->Self
    {
        let window = Window::new(title);
        Kiss3DSystem {
            window:window
        }
    }

    pub fn render(&mut self, context:&mut Context) {
        context.running = self.window.render();
    }
}

impl SimpleSystem for Kiss3DSystem
{
    fn update(&mut self, context:&mut Context) {
        self.render(context);
    }
}