use vecmath::vec2_normalized;

use crate::matrix::Matrix4x4;

fn unwrap3_or(x: Option<f32>, y: Option<f32>, z: Option<f32>, value: f32) -> (f32, f32, f32)
{
    let x = x.unwrap_or(value);
    let y = y.unwrap_or(value);
    let z = z.unwrap_or(value);
    (x, y, z)
}

/// Builds a scale matrix.
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Scale
{
    x: f32,
    y: f32,
    z: f32,
}

impl Scale
{
    pub fn id() -> ScaleBuilder
    {
        ScaleBuilder::default()
    }
}

impl ScaleBuilder
{
    /// Set the scaling along the x and y axis with a 2D vector.
    pub fn vec2(self, vector: &[f32; 2]) -> ScaleBuilder
    {
        self.x(vector[0]).y(vector[1])
    }
}

impl Matrix4x4 for ScaleBuilder
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        let (x, y, z) = unwrap3_or(self.x, self.y, self.z, 1.);
        [
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ]
    }
}

/// Builds a translation matrix.
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Translate
{
    x: f32,
    y: f32,
    z: f32,
}

impl Translate
{
    pub fn id() -> TranslateBuilder
    {
        TranslateBuilder::default()
    }
}

impl TranslateBuilder
{
    /// Set the translation along the x and y axis with a 2D vector.
    pub fn vec2(self, vector: &[f32; 2]) -> TranslateBuilder
    {
        self.x(vector[0]).y(vector[1])
    }
}

impl Matrix4x4 for TranslateBuilder
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        let (x, y, z) = unwrap3_or(self.x, self.y, self.z, 0.);
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [x, y, z, 1.],
        ]
    }
}

/// Builds a rotation matrix.
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Rotate
{
    cos: f32,
    sin: f32,
}

impl Rotate
{
    pub fn id() -> RotateBuilder
    {
        RotateBuilder::default()
    }
}

impl RotateBuilder
{
    pub fn radians(self, radians: f32) -> RotateBuilder
    {
        self.cos(radians.cos()).sin(radians.sin())
    }

    pub fn normalized_vec2(self, vector: [f32; 2]) -> RotateBuilder
    {
        self.cos(vector[0]).sin(vector[1])
    }

    pub fn vec2(self, vector: [f32; 2]) -> RotateBuilder
    {
        self.normalized_vec2(vec2_normalized(vector))
    }
}

impl Matrix4x4 for RotateBuilder
{
    fn into_matrix(self) -> [[f32; 4]; 4]
    {
        let c = self.cos.unwrap_or(1.);
        let s = self.sin.unwrap_or(0.);
        [
            [c, s, 0., 0.],
            [-s, c, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    }
}
