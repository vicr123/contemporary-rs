use gpui::Rgba;

pub trait Lerpable {
    fn lerp(self, with: Self, t: f32) -> Self;
}

macro_rules! default_lerp_impl {
    ($t:ty) => {
        impl Lerpable for $t {
            fn lerp(self, with: Self, t: f32) -> Self {
                let t: Self = t.into();
                if self < with {
                    (with - self.clone()) * t + self
                } else {
                    (self - with.clone()) * t + with
                }
            }
        }
    };
}

default_lerp_impl!(f32);
default_lerp_impl!(f64);

impl Lerpable for Rgba {
    fn lerp(self, with: Self, t: f32) -> Self {
        Self {
            r: self.r.lerp(with.r, t),
            g: self.g.lerp(with.g, t),
            b: self.b.lerp(with.b, t),
            a: self.a.lerp(with.a, t),
        }
    }
}
