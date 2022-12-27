use rayon::prelude::*;

pub fn present_value(rate: f64, future_cash_flow: f64, num_periods: f64) -> f64 {
    future_cash_flow / f64::powf(1_f64 + rate, num_periods)
}

pub fn future_value(rate: f64, present_cash_flow: f64, num_periods: f64) -> f64 {
    present_cash_flow * f64::powf(1_f64 + rate, num_periods)
}

pub fn net_present_value(rate: f64, cash_flows: &[f64]) -> f64 {
    cash_flows
        .par_iter()
        .enumerate()
        .map(|(index, cash_flow)| match index {
            0 => *cash_flow,
            _ => cash_flow / f64::powf(1_f64 + rate, index as f64),
        })
        .sum::<f64>()
}

pub fn round(value: f64) -> f64 {
    (value * 100_f64).round() / 100_f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_value_basic() {
        let rate = 0.05;
        let future_cash_flow = 1000.00;
        let num_periods = 4.00;
        let result = present_value(rate, future_cash_flow, num_periods);
        assert_eq!(result, 822.7024747918819);
    }

    #[test]
    fn future_value_basic() {
        let rate = 0.045;
        let present_cash_flow = 9000.00;
        let num_periods = 15.00;
        let result = future_value(rate, present_cash_flow, num_periods);
        assert_eq!(result, 17417.541987820376);
    }

    #[test]
    fn net_value_basic() {
        let rate = 0.1;
        let cash_flows: [f64; 4] = [-500000.00, 200000.00, 300000.00, 200000.00];
        let result = net_present_value(rate, &cash_flows);
        assert_eq!(result, 80015.02629601792);
    }
}
