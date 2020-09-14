
use crate::{SimpleSystem, components::Transform, components::Body};
use ncollide3d::shape::{ShapeHandle, Ball};
use legion::{query::*, Entity};
use nphysics3d::{world::*, object::DefaultBodySet, object::DefaultColliderSet, joint::DefaultJointConstraintSet, force_generator::DefaultForceGeneratorSet, object::RigidBodyDesc, object::ColliderDesc, object::BodyPartHandle, algebra::Velocity3};
use nalgebra as na;
use na::Vector3;


type Precision = f32;
pub struct BodySystem {
    mechanical_world:DefaultMechanicalWorld<Precision>,
    geometrical_world:DefaultGeometricalWorld<Precision>,
    bodies:DefaultBodySet<Precision>,
    colliders:DefaultColliderSet<Precision>,
    joint_constraints:DefaultJointConstraintSet<Precision>,
    force_generators:DefaultForceGeneratorSet<Precision>
}

impl BodySystem {

}

impl Default for BodySystem {
    fn default() -> Self {
        let gravity = -9.81;
        let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, gravity, 0.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        BodySystem {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators
        }
    }
}

impl SimpleSystem for BodySystem {
    fn once(&mut self, context:&mut crate::Context)
    {
        // do nothing is the default
    }

    fn update(&mut self, context:&mut crate::Context)
    {
        let bodies = &mut self.bodies;
        let mut q = <(Entity, &mut Transform, &mut Body)>::query();

        for (e, t, b) in q.iter_mut(&mut context.world ){
            if b.body_handle == None {
                let mut rigid_body_builder = RigidBodyDesc::<Precision>::new()
                .translation(t.position);

                rigid_body_builder.enable_gravity(true);

                let mut body = rigid_body_builder.build();
                let body_handle = self.bodies.insert(body);
                b.body_handle = Some(body_handle);
            }
        }

        self.mechanical_world.step(&mut self.geometrical_world, &mut self.bodies, &mut self.colliders, &mut self.joint_constraints, &mut self.force_generators);

        for (e, t, b) in q.iter_mut(&mut context.world ){
            if let Some(body_handle) = b.body_handle {
                let body = self.bodies.get(body_handle).unwrap();
                let part = body.part(0).unwrap();
                let translation = part.position().translation;
                
                t.position.x = translation.x;
                t.position.y = translation.y;
                t.position.z = translation.z;

                //t.position = t.position.add_to(rhs, out) + translation;
            }
        }
    }
}