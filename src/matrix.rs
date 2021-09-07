use std::{
    mem,
    ops::Range,
};

use ndarray::{
    arr2,
    Array2,
};

pub type Matrix4x4 = [[f32; 4]; 4];

pub enum Transform
{
    Id,
    Scale(Option<f32>, Option<f32>, Option<f32>),
    Translate(Option<f32>, Option<f32>, Option<f32>),
    Rotate(Option<f32>),
}

impl Transform
{
    pub fn matrix(&self) -> Matrix4x4
    {
        use Transform::*;

        fn unwrap3_or(x: Option<f32>, y: Option<f32>, z: Option<f32>, value: f32)
            -> (f32, f32, f32)
        {
            let x = x.unwrap_or(value);
            let y = y.unwrap_or(value);
            let z = z.unwrap_or(value);
            (x, y, z)
        }

        match self {
            &Id => [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
            &Scale(x, y, z) => {
                let (x, y, z) = unwrap3_or(x, y, z, 1.);
                [
                    [x, 0., 0., 0.],
                    [0., y, 0., 0.],
                    [0., 0., z, 0.],
                    [0., 0., 0., 1.],
                ]
            }
            &Translate(x, y, z) => {
                let (x, y, z) = unwrap3_or(x, y, z, 0.);
                [
                    [1., 0., 0., 0.],
                    [0., 1., 0., 0.],
                    [0., 0., 1., 0.],
                    [x, y, z, 1.],
                ]
            }
            &Rotate(radians) => {
                if let Some(radians) = radians {
                    let c = radians.cos();
                    let s = radians.sin();
                    [
                        [c, s, 0., 0.],
                        [-s, c, 0., 0.],
                        [0., 0., 1., 0.],
                        [0., 0., 0., 1.],
                    ]
                } else {
                    Id.matrix()
                }
            }
        }
    }

    pub fn arr2(&self) -> Array2<f32>
    {
        arr2(&self.matrix())
    }

    pub fn array(&self) -> [f32; 16]
    {
        unsafe { mem::transmute(self.matrix()) }
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Scale
{
    #[builder(setter(strip_option))]
    x: Option<f32>,

    #[builder(setter(strip_option))]
    y: Option<f32>,

    #[builder(setter(strip_option))]
    z: Option<f32>,
}

impl Scale
{
    pub fn id() -> ScaleBuilder
    {
        ScaleBuilder::default()
    }
}

#[allow(dead_code)]
impl ScaleBuilder
{
    pub fn transform(self) -> Transform
    {
        Transform::Scale(self.x.flatten(), self.y.flatten(), self.z.flatten())
    }

    pub fn matrix(self) -> [[f32; 4]; 4]
    {
        self.transform().matrix()
    }

    pub fn array(self) -> [f32; 16]
    {
        self.transform().array()
    }

    pub fn arr2(self) -> Array2<f32>
    {
        self.transform().arr2()
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Translate
{
    #[builder(setter(strip_option))]
    x: Option<f32>,

    #[builder(setter(strip_option))]
    y: Option<f32>,

    #[builder(setter(strip_option))]
    z: Option<f32>,
}

impl Translate
{
    pub fn id() -> TranslateBuilder
    {
        TranslateBuilder::default()
    }
}

#[allow(dead_code)]
impl TranslateBuilder
{
    pub fn transform(self) -> Transform
    {
        Transform::Translate(self.x.flatten(), self.y.flatten(), self.z.flatten())
    }

    pub fn matrix(self) -> [[f32; 4]; 4]
    {
        self.transform().matrix()
    }

    pub fn array(self) -> [f32; 16]
    {
        self.transform().array()
    }

    pub fn arr2(self) -> Array2<f32>
    {
        self.transform().arr2()
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Rotate
{
    #[builder(setter(strip_option))]
    radians: Option<f32>,
}

impl Rotate
{
    pub fn id() -> RotateBuilder
    {
        RotateBuilder::default()
    }
}

#[allow(dead_code)]
impl RotateBuilder
{
    pub fn transform(self) -> Transform
    {
        Transform::Rotate(self.radians.flatten())
    }

    pub fn matrix(self) -> [[f32; 4]; 4]
    {
        self.transform().matrix()
    }

    pub fn array(self) -> [f32; 16]
    {
        self.transform().array()
    }

    pub fn arr2(self) -> Array2<f32>
    {
        self.transform().arr2()
    }
}

#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
pub struct OrthographicProjectionMatrix
{
    abscissa: Range<f32>,
    ordinate: Range<f32>,
}

impl OrthographicProjectionMatrix
{
    pub fn builder() -> OrthographicProjectionMatrixBuilder
    {
        OrthographicProjectionMatrixBuilder::default()
    }
}

impl OrthographicProjectionMatrixBuilder
{
    pub fn build(&self) -> Matrix4x4
    {
        let x_min = self.abscissa.as_ref().map(|r| r.start).unwrap_or(-1.);
        let x_max = self.abscissa.as_ref().map(|r| r.end).unwrap_or(1.);
        let y_min = self.ordinate.as_ref().map(|r| r.start).unwrap_or(-1.);
        let y_max = self.ordinate.as_ref().map(|r| r.end).unwrap_or(1.);
        [
            [2. / (x_max - x_min), 0., 0., 0.],
            [0., 2. / (y_max - y_min), 0., 0.],
            [0., 0., 1., 0.],
            [
                -(x_max + x_min) / (x_max - x_min),
                -(y_max + y_min) / (y_max - y_min),
                0.,
                1.,
            ],
        ]
    }
}
