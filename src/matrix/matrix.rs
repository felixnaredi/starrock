use std::mem;

use ndarray::{
    arr2,
    Array2,
};

pub trait Matrix4x4: Sized
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        unsafe { mem::transmute(self.into_array()) }
    }

    fn into_array(self) -> [f32; 16]
    {
        unsafe { mem::transmute(self.into_matrix()) }
    }

    fn into_arr2(self) -> Array2<f32>
    {
        arr2(&self.into_matrix())
    }
}

pub struct Id;

impl Id
{
    pub fn new() -> Id
    {
        Id
    }
}

impl Matrix4x4 for Id
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    }
}
