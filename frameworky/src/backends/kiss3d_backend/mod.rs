use std::{collections::HashMap, f32::consts::PI, time::Instant};
//use nalgebra::{Point3, UnitQuaternion, Vector3};
use legion::*;
use kiss3d::{window::Window, light::Light, ncollide3d::math::Translation, scene::SceneNode, event::WindowEvent, event::Action, event::MouseButton, camera::ArcBall, nalgebra::Point3, nalgebra::UnitQuaternion, nalgebra::Vector3, window::State};

use crate::{SimpleSystem, Context, events::MouseEvent, events::MouseEventType, events::KeyEvent, Frameworky};
use crate::components::*;

//use super::arc_ball_modified::ArcBallModified;
pub struct Kiss3DBackend
{
    arc_ball_camera:ArcBall,
    nodes:HashMap<Entity, SceneNode>,
    frameworky:Frameworky
}


impl Kiss3DBackend
{
    pub fn start(frameworky:Frameworky, title:&str)
    {
        let arc_ball = ArcBall::new(
            Point3::new(0.0, 20.0, 20.0),
            Point3::origin());

        let window = Window::new(title);
        let backend = Kiss3DBackend {
            arc_ball_camera: arc_ball,
            nodes:HashMap::new(),
            frameworky
        };

        window.render_loop(backend);
    }

    fn sync_from(&mut self,  window:&mut Window) {
        let bodies = &mut self.nodes;
        let world = &mut self.frameworky.context.world;
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

    fn process_events(&mut self, window:&mut Window) {
        for e in window.events().iter() {
            match e.value 
            {
                WindowEvent::MouseButton(mb, a, _m) =>
                {
                    let pos = window.cursor_pos().unwrap();
                    let b = if mb == MouseButton::Button1 { 0 } else { 1 };

                    let e = MouseEvent
                    {
                        button:b,
                        event_type:if a == Action::Press { MouseEventType::ButtonDown } else { MouseEventType::ButtonUp },
                        screen_x:pos.0 as i32,
                        screen_y:pos.1 as i32
                    };

                    self.frameworky.context.push_event(e);
                },
                WindowEvent::Key(key, action, _modifier) => 
                {
                    let e = KeyEvent { 
                        down:if action == Action::Press { true } else { false },
                        key:key as u32
                    };

                    self.frameworky.context.push_event(e);
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

    pub fn render(&mut self, window:&mut Window) {
        //let now = Instant::now();
        self.sync_from(window);
       // context.once = self.window.render_with_camera(&mut self.arc_ball_camera);

       /* context.running = self.window.render_with_state(state)
        let took = now.elapsed().as_millis();
        let mut s = String::from("Sample render: ");
        s.push_str(&took.to_string());
        self.window.set_title(&s);*/
    }
}

impl State for Kiss3DBackend
{
    fn step(&mut self, window: &mut Window) {
        self.frameworky.update();
        self.process_events(window);
        self.sync_from(window);
    }
}