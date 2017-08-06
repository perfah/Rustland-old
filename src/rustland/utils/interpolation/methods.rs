use common::definitions::DefaultNumericType;

pub trait InterpolationMethod: Send{
    fn calc_progression(&self, x: DefaultNumericType) -> DefaultNumericType;
    fn get_left_bound(&self) -> DefaultNumericType;
    fn get_right_bound(&self) -> DefaultNumericType;
}

/// An interpolation method that keeps the motion speed at a constant rate.
pub struct LinearInterpolator;
impl InterpolationMethod for LinearInterpolator{
    fn calc_progression(&self, x: f32) -> f32 { x }
    fn get_left_bound(&self) -> DefaultNumericType { 0f32 }
    fn get_right_bound(&self) -> DefaultNumericType { 10f32 }
}


/// An interpolation method that accelerates the motion quadratically.
pub struct QuadraticInterpolator;
impl InterpolationMethod for QuadraticInterpolator{
    fn calc_progression(&self, x: f32) -> f32 { x * x }
    fn get_left_bound(&self) -> DefaultNumericType { 0f32 }
    fn get_right_bound(&self) -> DefaultNumericType { 10f32 }
}

/// An interpolation method that decelerates the motion.
pub struct SineInterpolator;
impl InterpolationMethod for SineInterpolator{
    fn calc_progression(&self, x: f32) -> f32 { x.sin() * x.sin() }
    fn get_left_bound(&self) -> DefaultNumericType { 0f32 }
    fn get_right_bound(&self) -> DefaultNumericType { 1.57f32 }
}