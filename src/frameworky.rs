use crate::{Context, SimpleSystem};

#[derive(Default)]
pub struct Frameworky
{
    pub systems:Vec<Box<dyn SimpleSystem>>,
    pub context:Context,
    pub on_before_fixed_update:Option<Box<dyn Fn(&mut Context)>>
}

impl Frameworky
{
    pub fn update(&mut self)
    {
        self.context.fixed_update_called = false;
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
            if let Some(f) = &self.on_before_fixed_update
            {
                f(&mut self.context);
            }
            for s in self.systems.iter_mut()
            {
                s.update_fixed(&mut self.context);
            }

            self.context.time.accumulator -= self.context.time.dt;
            self.context.time.t += self.context.time.dt;
            self.context.fixed_update_count += 1;
            self.context.fixed_update_called = true;
        }

        self.context.time.alpha = self.context.time.accumulator / self.context.time.dt;

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

    pub fn push_system<T:SimpleSystem + 'static + std::any::Any>(&mut self, s:T)
    {
        self.systems.push(Box::new(s));
    }

    pub fn get_system<T>(&self) -> Option<&T> where T : SimpleSystem
    {
        for s in self.systems.iter()
        {
            if let Some(any) = s.as_any()
            {
                if let Some(concrete) = any.downcast_ref::<T>()
                {
                    return Some(concrete);
                }
            }
        }
        None
    }

    pub fn get_system_mut<T>(&mut self) -> Option<&mut T> where T : SimpleSystem
    {
        for s in self.systems.iter_mut()
        {
            if let Some(any) = s.as_any_mut()
            {
                if let Some(concrete) = any.downcast_mut::<T>()
                {
                    return Some(concrete);
                }
            }
        }
        None
    }
}