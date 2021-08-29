fn f32mod(x: &mut f32, m: f32) -> bool
{
    if 0. > *x || *x > m {
        *x -= (*x / m).floor() * m;
        true
    } else {
        false
    }
}

pub fn position_modulo(position: &mut [f32; 2]) -> bool
{
    f32mod(&mut position[0], 4.) || f32mod(&mut position[1], 3.)
}
