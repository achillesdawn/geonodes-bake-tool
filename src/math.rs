use std::ops::{Add, Div, Mul, Sub};

pub fn map_range<T: Copy>(value: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}