use std::marker::{Send, PhantomData};
use num::{abs, FromPrimitive};

use common::definitions::DefaultNumericType;
use utils::interpolation::methods::InterpolationMethod;

pub mod methods;

pub struct NumericInterpolation{
    interpolation_method: Box<InterpolationMethod>,
    start_pole: DefaultNumericType,
    end_pole: DefaultNumericType, 
    pub intervals: u64,
    internal_progression: DefaultNumericType,
    ongoing: bool
}

impl NumericInterpolation{
    pub fn new(interpolation_method: Box<InterpolationMethod>, start_pole: DefaultNumericType, end_pole: DefaultNumericType, intervals: u64) -> NumericInterpolation{
        let (left_bound, right_bound) = (interpolation_method.get_left_bound(), interpolation_method.get_right_bound());
        
        assert!(left_bound < right_bound, "The left bound number must be smaller than the right!");

        NumericInterpolation{
            interpolation_method: interpolation_method,
            start_pole: start_pole,
            end_pole: end_pole,
            intervals: intervals,
            internal_progression: left_bound,
            ongoing: true
        }
    }

    pub fn next<T: FromPrimitive>(&mut self, output: &mut T) -> bool{
        let (left_bound, right_bound) = (self.interpolation_method.get_left_bound(), self.interpolation_method.get_right_bound());

        // Current progress between 0 and 1 decimally:
        let progress = abs(
            self.interpolation_method.calc_progression(self.internal_progression) / 
            (self.interpolation_method.calc_progression(right_bound) - self.interpolation_method.calc_progression(left_bound))
        );

        // Update the targeted outer value to the current iteration value:start_pole
        *output = T::from_f32(self.start_pole + progress * (self.end_pole - self.start_pole)).unwrap();

        // Jump to the next intervall
        self.internal_progression += abs(right_bound - left_bound) / (self.intervals as f32);
        self.ongoing = progress < 1f32 && self.internal_progression < right_bound;

        if !self.ongoing {
            self.internal_progression = right_bound;
            *output = T::from_f32(self.end_pole).unwrap();
        }

        self.ongoing
    }

    pub fn reset(&mut self){
        self.internal_progression = 0f32;
    }

    pub fn is_ongoing(&self) -> bool{
        self.ongoing
    }
}

mod test{
    #[test]
    fn numeric_interpolation_test(){
        use num::abs;
        use super::NumericInterpolation;
        use super::methods::{SigmoidInterpolator, QuadraticInterpolator};

        let iterations = 100;
        let (start_pole, end_pole) = (0f32, 100f32);

        /*
            Run 'cargo test -- --nocapture' for a visual demonstration.
        */

        // Interpolation descriptions:
        let mut interpolator = NumericInterpolation::new(box SigmoidInterpolator{}, start_pole, end_pole, iterations);

        let mut pos = 0;
        let mut current_iter = 0;
        loop {
            let ongoing = interpolator.next(&mut pos);
            //let cond2 = quad.next(&mut q);
            if !ongoing { break; }

            print!("it. {}: ", current_iter);

            for i in 1..pos { print!(" "); }
                print!("X");


            if current_iter == iterations / 2 {
                // Sigmoid at x=0
                assert!(pos == end_pole as i32 / 2 );
            }
            
                
            println!();
            current_iter += 1;
        }
    }
}