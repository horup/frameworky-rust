use crate::{Context, SimpleSystem};

pub struct Time
{
    pub t:f64,
    pub dt:f64,
    pub current:f64,
    pub accumulator:f64
}

impl Default for Time
{
    fn default() -> Self {
        let dt = 1.0 / 20.0; 
        Time {
            t:0.0,
            dt:dt,
            current:instant::now() / 1000.0,
            accumulator:0.0
        }
    }
}

#[derive(Default)]
pub struct Frameworky
{
    pub systems:Vec<Box<dyn SimpleSystem>>,
    pub context:Context,
    pub time:Time
    
}

impl Frameworky
{
    pub fn update(&mut self)
    {
        let new_time = instant::now() / 1000.0 as f64;
        let update_time = new_time - self.time.current;
        self.time.current = new_time;
        self.time.accumulator += update_time;
        if self.context.once == false
        {
            for s in self.systems.iter_mut()
            {
                s.once(&mut self.context);
            }

            self.context.once = true;
        }

        while self.time.accumulator >= self.time.dt
        {
            for s in self.systems.iter_mut()
            {
                s.update_fixed(&mut self.context);
            }

            self.time.accumulator -= self.time.dt;
            self.time.t += self.time.dt;
        }

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
    }

    pub fn push_system<T:SimpleSystem + 'static>(&mut self, s:T)
    {
        self.systems.push(Box::new(s));
    }
}