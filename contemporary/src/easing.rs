pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t * 2.0;
    if t < 1. {
        0.5 * t * t * t
    } else {
        let t = t - 2.;
        0.5 * (t * t * t + 2.)
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.;
    t * t * t + 1.
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}
