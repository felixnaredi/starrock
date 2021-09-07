use std::ops::Range;

use crate::matrix::Matrix4x4;

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(skip))]
#[builder(pattern = "owned")]
pub struct OrthographicProjection
{
    abscissa: Range<f32>,
    ordinate: Range<f32>,
}

impl OrthographicProjection
{
    pub fn default() -> OrthographicProjectionBuilder
    {
        OrthographicProjectionBuilder::default()
    }
}

impl OrthographicProjectionBuilder
{
    pub fn build(self) -> OrthographicProjection
    {
        OrthographicProjection {
            abscissa: self.abscissa.unwrap_or(-1. ..1.),
            ordinate: self.ordinate.unwrap_or(-1. ..1.),
        }
    }
}

impl Matrix4x4 for OrthographicProjection
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        let x_min = self.abscissa.start;
        let x_max = self.abscissa.end;
        let y_min = self.ordinate.start;
        let y_max = self.ordinate.end;
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

impl Matrix4x4 for OrthographicProjectionBuilder
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        self.build().into_matrix()
    }
}
