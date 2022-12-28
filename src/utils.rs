const PRECISION: f64 = 0.0000001;
const MAX_ITERATIONS: u32 = 20;
const DEDAULT_GUESS: f64 = 0.1;

pub fn find_root<F>(func: F, guess: Option<f64>) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let mut guess = guess.unwrap_or(DEDAULT_GUESS);
    let derivative_func =
        |guess: f64| (func(guess + PRECISION) - func(guess - PRECISION)) / (2.0 * PRECISION);

    for _ in 1..MAX_ITERATIONS {
        let result = func(guess);
        let derivative_result = derivative_func(guess);
        let next_guess = guess - result / derivative_result;

        if (next_guess - guess).abs() <= PRECISION {
            return Some(next_guess);
        }

        guess = next_guess;
    }

    None
}
