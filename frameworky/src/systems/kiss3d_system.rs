use std::collections::HashMap;
use nalgebra::Point3;
use legion::*;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, ncollide3d::math::Vector, camera::ArcBall, scene::SceneNode};

use crate::{SimpleSystem, Context};
use crate::components::*;
pub struct Kiss3DSystem
{
    window:Window,
    arc_ball_camera:ArcBall,
    bodies:HashMap<Entity, SceneNode>
}
type RenderableEntity<'a, E> = (E, &'a mut Transform, &'a mut Body);
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
            arc_ball_camera: arc_ball,
            bodies:HashMap::new()
        }
    }

    fn sync_from(&mut self, context:&mut Context) {
        let window = &mut self.window;
        let bodies = &mut self.bodies;
        let world = &mut context.world;
        let col = || rand::random::<f32>();
        let test = Transform::new(0.0, 0.0, 0.0);
        test.position.x;

        <(Entity, &mut Transform, &mut Body)>::query().for_each_mut(world, |(k, t, b)| {
            if !bodies.contains_key(k) {
                
                let mut sphere = window.add_sphere(0.5);
                sphere.set_color(col(), col(), col());
                bodies.insert(*k, sphere);
            }

            // and they are syncronized 
            let sphere = bodies.get_mut(k).unwrap();
            let p = &t.position;
            sphere.set_local_translation(Translation::new(p.x, p.y, p.z));
        });
    }

    pub fn render(&mut self, context:&mut Context) {
        self.sync_from(context);
        context.running = self.window.render_with_camera(&mut self.arc_ball_camera);
    }
}

impl SimpleSystem for Kiss3DSystem
{
    fn once(&mut self, context:&mut Context) {
        let w = &mut self.window;
        
        w.set_light(Light::StickToCamera);
    }
    fn update(&mut self, context:&mut Context) {
     

        self.render(context)
    }
}