use crate::{Context, SimpleSystem};

#[derive(Default)]
pub struct Frameworky
{
    pub systems:Vec<Box<dyn SimpleSystem>>,
    pub context:Context
}

impl Frameworky
{
    pub fn update(&mut self)
    {
        let new_time = instant::now() / 1000.0 as f64;
        let mut update_time = new_time - self.context.time.current;
        update_time = if update_time > 0.25 {0.25} else {update_time};
        self.context.time.current = new_time;
        self.context.time.accumulator += update_time;
        if self.context.once == false
        {
            for s in self.systems.iter_mut()
            {
                s.once(&mut self.context);
            }

            self.context.once = true;
        }

        while self.context.time.accumulator >= self.context.time.dt
        {
            for s in self.systems.iter_mut()
            {
                s.update_fixed(&mut self.context);
            }

            self.context.time.accumulator -= self.context.time.dt;
            self.context.time.t += self.context.time.dt;
        }

        self.context.time.alpha = self.context.time.accumulator / self.context.time.dt;
        println!("{}", self.context.time.alpha);

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