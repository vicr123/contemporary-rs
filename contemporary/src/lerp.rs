use gpui::{Bounds, Pixels, Point, Rgba, Size};
use std::fmt::Debug;

pub trait Lerpable {
    fn lerp(&self, with: &Self, t: f32) -> Self;
}

macro_rules! default_lerp_impl {
    ($t:ty) => {
        impl Lerpable for $t {
            fn lerp(&self, with: &Self, t: f32) -> Self {
                let t: Self = t.into();
                (with - self.clone()) * t + self
            }
        }
    };
}

default_lerp_impl!(f32);
default_lerp_impl!(f64);

impl Lerpable for Rgba {
    fn lerp(&self, with: &Self, t: f32) -> Self {
        Self {
            r: self.r.lerp(&with.r, t),
            g: self.g.lerp(&with.g, t),
            b: self.b.lerp(&with.b, t),
            a: self.a.lerp(&with.a, t),
        }
    }
}

impl<T> Lerpable for Bounds<T>
where
    T: Clone + PartialEq + Default + Debug + Lerpable,
{
    fn lerp(&self, with: &Self, t: f32) -> Self {
        Self {
            origin: self.origin.lerp(&with.origin, t),
            size: self.size.lerp(&with.size, t),
        }
    }
}

impl<T> Lerpable for Point<T>
where
    T: Clone + PartialEq + Default + Debug + Lerpable,
{
    fn lerp(&self, with: &Self, t: f32) -> Self {
        Self {
            x: self.x.lerp(&with.x, t),
            y: self.y.lerp(&with.y, t),
        }
    }
}

impl<T> Lerpable for Size<T>
where
    T: Clone + PartialEq + Default + Debug + Lerpable,
{
    fn lerp(&self, with: &Self, t: f32) -> Self {
        Self {
            width: self.width.lerp(&with.width, t),
            height: self.height.lerp(&with.height, t),
        }
    }
}

impl Lerpable for Pixels {
    fn lerp(&self, with: &Self, t: f32) -> Self {
        (*with - *self) * t + *self
    }
}
