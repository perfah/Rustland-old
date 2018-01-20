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
        use super::methods::{LinearInterpolator, QuadraticInterpolator, DecelerationInterpolator};

        let iterations = 100;
        let (start_pole, end_pole) = (0f32, 100f32);

        /*
            Run with '--nocapture' as argument for a visual demonstration.
            linear: l(x) = x, 
            quad: q(x) = x*x
            x = 1 <=> l(x) = q(x) 
        */

        // Output of l(x), q(x)
        let (mut l, mut q) = (0, 0);

        // Interpolation descriptions:
        let mut linear = NumericInterpolation::new(box LinearInterpolator{}, start_pole, end_pole, iterations);
        let mut quad = NumericInterpolation::new(box QuadraticInterpolator{}, start_pole, end_pole, iterations);

        // On iteration num. 10 linear and quad meet: l(x) = q(x)
        for i in 0..10{        
            linear.next(&mut l);
            quad.next(&mut q);
        }
        //assert!(abs(l - q) <= 1); // rounding error are tolerated

        // Visual demonstration of l(x) and q(x):
        linear.reset();
        quad.reset();
        let mut current_iter = 0;
        loop {
            let cond1 = linear.next(&mut l);
            let cond2 = quad.next(&mut q);
            if !(cond1 || cond2) { break; }

            print!("it. {}: ", current_iter);

            if l == q{
                // Here once again they meet.
                for i in 1..l { print!(" "); }
                print!("X");
            }
            else if l < q { 
                for i in 1..l { print!(" "); }
                print!("l");

                for i in 1..(q-l) { print!(" "); }
                print!("q");
            }
            else{
                for i in 1..q{ print!(" "); }
                print!("q");

                for i in 1..(l-q){ print!(" "); }
                print!("l");
            }

            println!();
            current_iter += 1;
        }
    }
}