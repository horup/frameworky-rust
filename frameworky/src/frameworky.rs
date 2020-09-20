use kiss3d::window::{State, Window};

use crate::{Context, SimpleSystem, systems::Kiss3DSystem};

pub struct Frameworky
{
    pub systems:Vec<Box<dyn SimpleSystem>>,
    pub context:Context
}

impl<'a> Default for Frameworky
{
    fn default() -> Self {

        Self {
            context: Context::default(),
            systems: Vec::default()
        }
    }
}

impl State for Frameworky
{
    fn step(&mut self, window: &mut Window) {
        if self.context.once == false
        {
            for s in self.systems.iter_mut()
            {
                s.once(&mut self.context);
            }

            self.context.once = true;
        }

        for s in self.systems.iter_mut()
        {

            s.update(&mut self.context);
            s.update_with_window(&mut self.context, window);
        }

        let events = self.context.events.clone();
        self.context.events.clear();

        for s in self.systems.iter_mut()
        {
            for e in events.iter() {
                s.execute(&mut self.context, e.as_ref());
            }
        }
    }
}


impl Frameworky
{
    pub fn push_system<T:SimpleSystem + 'static>(&mut self, s:T)
    {
        self.systems.push(Box::new(s));
    }

    pub fn start_loop(frameworky:Frameworky)
    {
        let window = Window::new("Kiss3d: wasm example");        
        window.render_loop(frameworky);
    /*    for s in self.systems.iter_mut()
        {
            s.once(&mut self.context);
        }

        loop {
            for s in self.systems.iter_mut()
            {
                s.update(&mut self.context);
            }

            let events = self.context.events.clone();
            self.context.events.clear();

            for s in self.systems.iter_mut()
            {
                for e in events.iter() {
                    s.execute(&mut self.context, e.as_ref());
                }
            }

            if !self.context.running {
                return;
            }
        }*/
    }
}