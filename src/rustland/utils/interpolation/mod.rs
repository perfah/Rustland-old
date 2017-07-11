use std::marker::PhantomData;
use num::{abs, FromPrimitive};

pub mod presets;

trait InterpolationMethod{
    const left_bound: f32;
    const right_bound: f32;

    fn interpolate(x: f32) -> f32;
}

struct NumericInterpolation<I: InterpolationMethod>{
    interpolation_method: PhantomData<I>,
    start_pole: f32,
    end_pole: f32, 
    linear_iterations: u32,
    internal_progression: f32
}

impl<M: InterpolationMethod> NumericInterpolation<M>{
    pub fn new(start_pole: f32, end_pole: f32, linear_iterations: u32) -> NumericInterpolation<M>{
        assert!(M::left_bound < M::right_bound, "The left bound number must be smaller than the right!");
        
        NumericInterpolation::<M>{
            interpolation_method: PhantomData,
            start_pole: start_pole,
            end_pole: end_pole,
            linear_iterations: linear_iterations,
            internal_progression: M::left_bound
        }
    }

    pub fn next<T: FromPrimitive>(&mut self, output: &mut T) -> bool{
        // Current progress between 0 and 1 decimally:
        let progress = abs(M::interpolate(self.internal_progression) / (M::right_bound - M::left_bound));

        // Update the targeted outer value to the current iteration value:
        *output = T::from_f32(self.start_pole + progress * (self.end_pole - self.start_pole)).unwrap();

        // Jump to the next iteration
        self.internal_progression += abs(M::right_bound - M::left_bound) / (self.linear_iterations as f32);
        
        if progress < 1f32 && self.internal_progression < M::right_bound { true }
        else{
            self.internal_progression = M::right_bound;
            *output = T::from_f32(self.end_pole).unwrap();

            false
        }
    }

    pub fn reset(&mut self, ){
        self.internal_progression = 0f32;
    }
}

mod test{
    #[test]
    fn numeric_interpolation_test(){
        use num::abs;
        use super::NumericInterpolation;
        use super::presets::{LinearInterpolator, QuadraticInterpolator, DecelerationInterpolator};

        let iterations = 100;
        let (start_pole, end_pole) = (0f32, 100f32);

        /*
            Run with '--nocapture' as argument for a visual demonstration of the quad.
            linear: l(x) = x, 
            quad: q(x) = x*x
            x = 1 <=> l(x) = q(x) 
        */

        // Output of l(x), q(x)
        let (mut l, mut q) = (0, 0);

        // Interpolation descriptions:
        let mut linear = NumericInterpolation::<LinearInterpolator>::new(start_pole, end_pole, iterations);
        let mut quad = NumericInterpolation::<QuadraticInterpolator>::new(start_pole, end_pole, iterations);

        // On iteration num. 10 linear and quad meet: l(x) = q(x)
        for i in 0..10{        
            linear.next(&mut l);
            quad.next(&mut q);
        }
        assert!(abs(l - q) <= 1); // rounding error are tolerated

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