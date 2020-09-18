use std::{collections::HashMap, f32::consts::PI, time::Instant};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use legion::*;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, scene::SceneNode, event::WindowEvent, event::Action, event::MouseButton};

use crate::{SimpleSystem, Context, events::MouseButtonDown, events::MouseButtonUp, events::KeyEvent};
use crate::components::*;

use super::arc_ball_modified::ArcBallModified;
pub struct Kiss3DSystem
{
    window:Window,
    arc_ball_camera:ArcBallModified,
    nodes:HashMap<Entity, SceneNode>
}
impl Kiss3DSystem
{
    pub fn new(title:&str)->Self
    {
        let mut window = Window::new(title);
        window.set_framerate_limit(Some(60));

        let arc_ball = ArcBallModified::new(
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
/*
                    let mut quad = window.add_quad(1.0, 1.0, 1, 1);
                    quad.set_color(col(), col(), col());
                    bodies.insert(*k, quad);*/
                }
                else if b.shape == Shape::Plane {
                    let size = 100.0;
                    let mut plane = window.add_quad(size, size, 1, 1);
                    let rot = UnitQuaternion::from_axis_angle(&Vector3::<f32>::x_axis(), PI / 2.0);
                    plane.append_rotation(&rot);
                    bodies.insert(*k, plane);
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
            match e.value 
            {
                WindowEvent::MouseButton(mb, a, _m) =>
                {
                    let pos = self.window.cursor_pos().unwrap();
                    let b = if mb == MouseButton::Button1 { 0 } else { 1 };
                    if a == Action::Press 
                    {
                        let e = MouseButtonDown 
                        {
                            button:b,
                            screen_x:pos.0,
                            screen_y:pos.1
                        };

                        context.push_event(e);
                    }
                    else 
                    {
                        let e = MouseButtonUp 
                        {
                            button:b,
                            screen_x:pos.0,
                            screen_y:pos.1
                        };

                        context.push_event(e);
                    }
                },
                WindowEvent::Key(key, action, modifier) => 
                {
                    let e = KeyEvent { 
                        down:if action == Action::Press { true } else { false },
                        key:key as u32
                    };

                    context.push_event(e);
                   /* if action == Action::Press 
                    {
                        let index:i32 = key as i32;
                        let e = KeyEvent {

                        }
                    }
                    else 
                    {

                    }*/


                }
                _ => {}
            }
        }
    }

    pub fn render(&mut self, context:&mut Context) {
        let now = Instant::now();
        self.sync_from(context);
        context.running = self.window.render_with_camera(&mut self.arc_ball_camera);

        let took = now.elapsed().as_millis();
        let mut s = String::from("Sample render: ");
        s.push_str(&took.to_string());
        self.window.set_title(&s);
    }
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