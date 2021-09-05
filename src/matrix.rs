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
    #[builder(default = "-1.")]
    x_min: f32,

    #[builder(default = "1.")]
    x_max: f32,

    #[builder(default = "-1.")]
    y_min: f32,

    #[builder(default = "1.")]
    y_max: f32,
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
        let x_min = self.x_min.unwrap();
        let x_max = self.x_max.unwrap();
        let y_min = self.y_min.unwrap();
        let y_max = self.y_max.unwrap();
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
