use std::{collections::HashMap, f32::consts::PI};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use legion::*;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, camera::ArcBall, scene::SceneNode};

use crate::{SimpleSystem, Context};
use crate::components::*;
pub struct Kiss3DSystem
{
    window:Window,
    arc_ball_camera:ArcBall,
    nodes:HashMap<Entity, SceneNode>
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
            nodes:HashMap::new()
        }
    }

    fn sync_from(&mut self, context:&mut Context) {
        let window = &mut self.window;
        let bodies = &mut self.nodes;
        let world = &mut context.world;
        let col = || rand::random::<f32>();
        let test = Transform::new(0.0, 0.0, 0.0);
        test.position.x;

        <(Entity, &mut Transform, &mut Body)>::query().for_each_mut(world, |(k, t, b)| {
            if !bodies.contains_key(k) {
                if b.shape == Shape::Sphere {
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_color(col(), col(), col());
                    bodies.insert(*k, sphere);
                }
                else if b.shape == Shape::Plane {
                    let size = 100.0;
                    let mut plane = window.add_quad(size, size, 1, 1);
                    let rot = UnitQuaternion::from_axis_angle(&Vector3::<f32>::x_axis(), PI / 2.0);
                    plane.append_rotation(&rot);
                }
            }

            if let Some(node) = bodies.get_mut(k) {
                let p = &t.position;
                node.set_local_translation(Translation::new(p.x, p.y, p.z));
            }
           
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