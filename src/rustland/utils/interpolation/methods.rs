use common::definitions::DefaultNumericType;

pub trait InterpolationMethod: Send{
    const left_bound: DefaultNumericType;
    const right_bound: DefaultNumericType;

    fn calc_progression(&self, x: DefaultNumericType) -> DefaultNumericType;
    fn get_left_bound(&self) -> DefaultNumericType { Self::left_bound }
    fn get_right_bound(&self) -> DefaultNumericType { Self::right_bound }
}

/// An interpolation method that keeps the motion speed at a constant rate.
pub struct LinearInterpolator;
impl InterpolationMethod for LinearInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 10f32;

    fn calc_progression(&self, x: f32) -> f32 { x }
}


/// An interpolation method that accelerates the motion quadratically.
pub struct QuadraticInterpolator;
impl InterpolationMethod for QuadraticInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 10f32;

    fn calc_progression(&self, x: f32) -> f32 { x * x }
}

/// An interpolation method that decelerates the motion.
pub struct DecelerationInterpolator;
impl InterpolationMethod for DecelerationInterpolator{
    const left_bound: DefaultNumericType = 0f32;
    const right_bound: DefaultNumericType = 1.57f32;

    fn calc_progression(&self, x: f32) -> f32 { x.sin() * x.sin() }
}