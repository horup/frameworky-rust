use nalgebra::Vector3;


#[derive(Clone, Copy, Debug, PartialEq, Default)]

pub struct Transform
{
    pub position:Vector3<f32>
}

impl Transform
{
    pub fn new(px:f32, py:f32, pz:f32) -> Transform
    {
        Transform {position:Vector3::new(px, py, pz)}
    }
}