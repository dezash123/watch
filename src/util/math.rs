fn clamp(num: f32, min: f32, max: f32) -> f32 {
    if num > max {
        max
    } else if num < min {
        min
    } else {
        num
    }
}

pub fn clamp_upct(num: f32) -> f32 {
    clamp(num, 0.0, 1.0)
}

pub fn clamp_ipct(num: f32) -> f32 {
    clamp(num, -1.0, 1.0)
}
