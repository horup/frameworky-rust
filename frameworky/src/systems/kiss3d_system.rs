use std::{collections::HashMap, f32::consts::PI};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use legion::*;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, camera::ArcBall, scene::SceneNode, event::WindowEvent, event::Action, event::MouseButton};

use crate::{SimpleSystem, Context, Event};
use crate::components::*;
pub struct Kiss3DSystem
{
    window:Window,
    arc_ball_camera:ArcBall,
    nodes:HashMap<Entity, SceneNode>
}
impl Kiss3DSystem
{
    pub fn new(title:&str)->Self
    {
        let mut window = Window::new(title);
        window.set_framerate_limit(Some(60));

        let arc_ball = ArcBall::new(
            Point3::new(0.0, 20.0, 20.0),
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

    fn process_events(&mut self, context:&mut Context) {
        for e in self.window.events().iter() {
            match e.value {
                WindowEvent::MouseButton(mb, a, m) =>{
                    let pos = self.window.cursor_pos().unwrap();
                    let b = if mb == MouseButton::Button1 { 0 } else { 1 };
                    if a == Action::Press {
                        let e = MouseButtonDown {
                            button:b,
                            screen_x:pos.0,
                            screen_y:pos.1
                        };

                        context.push_event(e);
                    }
                    else {
                        let e = MouseButtonUp {
                            button:b,
                            screen_x:pos.0,
                            screen_y:pos.1
                        };

                        context.push_event(e);
                    }
                },
                _ => {}
            }
        }
    }

    pub fn render(&mut self, context:&mut Context) {
        self.sync_from(context);

       
        context.running = self.window.render_with_camera(&mut self.arc_ball_camera);
    }
}

#[derive(Debug, Default)]
pub struct MouseButtonDown {
    screen_x:f64,
    screen_y:f64,
    button:u32
}
impl Event for MouseButtonDown {
}

#[derive(Debug, Default)]
pub struct MouseButtonUp {
    screen_x:f64,
    screen_y:f64,
    button:u32
}
impl Event for MouseButtonUp {
}


impl SimpleSystem for Kiss3DSystem
{
    fn once(&mut self, _context:&mut Context) {
        let w = &mut self.window;
        
        w.set_light(Light::StickToCamera);
    }
    fn update(&mut self, context:&mut Context) {
        self.process_events(context);
        self.render(context)
    }
}