use std::ops::Range;

pub type Matrix4x4 = [[f32; 4]; 4];

/// Scale matrix for x and y-axis.
pub fn scale_xy(x: f32, y: f32) -> Matrix4x4
{
    [
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

/// Translation matrix for x and y-axis.
pub fn translate_xy(x: f32, y: f32) -> Matrix4x4
{
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [x, y, 0., 1.],
    ]
}

/// Rotation matrix that rotates x and y by `radians`.
pub fn rotate_xy(radians: f32) -> Matrix4x4
{
    [
        [radians.cos(), radians.sin(), 0., 0.],
        [-radians.sin(), radians.cos(), 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
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
