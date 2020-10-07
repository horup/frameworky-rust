use nalgebra::Vector3;

use super::Precision;

#[derive(Copy, Clone, Debug)]
pub struct Camera
{
    pub direction:Vector3<Precision>
}

impl Default for Camera 
{
    fn default() -> Self {
        Camera {
            direction:Vector3::new(0.0, -1.0, -1.0)
        }   
    }
}