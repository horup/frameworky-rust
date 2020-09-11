use nalgebra::Point3;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, ncollide3d::math::Vector, camera::ArcBall};

use crate::{SimpleSystem, Context};

pub struct Kiss3DSystem
{
    window:Window,
    arc_ball_camera:ArcBall
}

impl Kiss3DSystem
{
    pub fn new(title:&str)->Self
    {
        let window = Window::new(title);
        let arc_ball = ArcBall::new(
            Point3::new(0.0, 10.0, 10.0),
            Point3::origin());
        Kiss3DSystem {
            window,
            arc_ball_camera: arc_ball
        }
    }

    pub fn render(&mut self, context:&mut Context) {
        context.running = self.window.render_with_camera(&mut self.arc_ball_camera);
    }
}

impl SimpleSystem for Kiss3DSystem
{
    fn once(&mut self, context:&mut Context) {
        let w = &mut self.window;
       
        {
            let mut c = w.add_cube(1.0, 1.0, 1.0);
            c.set_color(1.0, 1.0, 1.0);
        }
        {
            let mut c = w.add_cube(1.0, 1.0, 1.0);
            c.set_color(1.0, 1.0, 1.0);
            c.set_local_translation(Translation::new(2.0, 0.0, 0.0));
        }

       // w.set_light(Light::StickToCamera);
    }
    fn update(&mut self, context:&mut Context) {
        //self.render(context);
        self.render(context)
    }
}