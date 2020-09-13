use crate::{SimpleSystem, components::Transform, components::Body};
use legion::{query::*, Entity};
use nphysics3d::{world::*, object::DefaultBodySet, object::DefaultColliderSet, joint::DefaultJointConstraintSet, force_generator::DefaultForceGeneratorSet};
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
        let mut q = <(Entity, &mut Transform, &mut Body)>::query();

    }
}