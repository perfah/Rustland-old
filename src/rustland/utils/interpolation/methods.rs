use common::definitions::DefaultNumericType;
use utils::interpolation::InterpolationMethod;

/// An interpolation method that keeps the motion speed at a constant rate.
pub struct LinearInterpolator;
impl InterpolationMethod for LinearInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 10f32;

    fn interpolate(&self, x: f32) -> f32 { x }
}

/// An interpolation method that accelerates the motion quadratically.
pub struct QuadraticInterpolator;
impl InterpolationMethod for QuadraticInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 10f32;

    fn interpolate(&self, x: f32) -> f32 { x * x }
}

/// An interpolation method that decelerates the motion.
pub struct DecelerationInterpolator;
impl InterpolationMethod for DecelerationInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 1.57f32;

    fn interpolate(&self, x: f32) -> f32 { x.sin() }
}