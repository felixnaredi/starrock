use std::ops::Range;

use crate::matrix::Matrix4x4;

#[derive(Builder, Debug)]
#[builder(build_fn(skip))]
#[builder(pattern = "owned")]
pub struct OrthographicProjectionMatrix
{
    abscissa: Range<f32>,
    ordinate: Range<f32>,
}

impl OrthographicProjectionMatrix
{
    pub fn default() -> OrthographicProjectionMatrixBuilder
    {
        OrthographicProjectionMatrixBuilder::default()
    }
}

impl Matrix4x4 for OrthographicProjectionMatrixBuilder
{
    fn matrix(self) -> [[f32; 4]; 4]
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
