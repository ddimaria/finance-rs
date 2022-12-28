use rayon::prelude::*;

mod utils;

pub fn present_value(rate: f64, future_cash_flow: f64, num_periods: f64) -> f64 {
    future_cash_flow / f64::powf(1.0 + rate, num_periods)
}

pub fn future_value(rate: f64, present_cash_flow: f64, num_periods: f64) -> f64 {
    present_cash_flow * f64::powf(1.0 + rate, num_periods)
}

pub fn net_present_value(rate: f64, cash_flows: &[f64]) -> f64 {
    cash_flows
        .par_iter()
        .enumerate()
        .map(|(index, cash_flow)| match index {
            0 => *cash_flow,
            _ => cash_flow / f64::powf(1.0 + rate, index as f64),
        })
        .sum::<f64>()
}

pub fn internal_rate_of_return(cash_flows: &[f64], guess: Option<f64>) -> Option<f64> {
    let npv = |rate: f64| net_present_value(rate, cash_flows);

    utils::find_root(npv, guess)
}

pub fn payback_period(cash_flows: &[f64], num_periods: f64) -> f64 {
    // for even cash flows
    if num_periods == 0.0 {
        return (cash_flows[0] / cash_flows[1]).abs();
    }

    // for uneven cash flows
    let mut cumulative_cash_flow = cash_flows[0];
    let mut years = 1.0;

    for cash_flow in cash_flows.iter().skip(1) {
        cumulative_cash_flow += cash_flow;

        if cumulative_cash_flow > 0.0 {
            return years + (cumulative_cash_flow - cash_flow) / cash_flow;
        } else {
            years += 1.0;
        }
    }

    years
}

pub fn return_on_investment(present_cash_flow: f64, earnings: f64) -> f64 {
    (earnings - present_cash_flow.abs()) / present_cash_flow.abs()
}

pub enum AmortizationPeriod {
    Month,
    Year,
}

pub fn amortization(
    principal: f64,
    rate: f64,
    num_periods: f64,
    amortization_period: AmortizationPeriod,
    pay_at_beginning: bool,
) -> f64 {
    let rate_per_period = rate / 12.0 / 100.0;
    let multiplier = match amortization_period {
        AmortizationPeriod::Month => 1.0,
        AmortizationPeriod::Year => 12.0,
    };
    let mut num_interest_accruals = num_periods * multiplier;

    if pay_at_beginning {
        num_interest_accruals -= 1.0;
    }

    let numerator = rate_per_period * f64::powf(1.0 + rate_per_period, num_interest_accruals);
    let denominator = f64::powf(1.0 + rate_per_period, num_periods * multiplier) - 1.0;

    principal * (numerator / denominator)
}

pub fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn round_decimal(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_value_basic() {
        let rate = 0.05;
        let future_cash_flow = 1000.0;
        let num_periods = 4.0;
        let result = present_value(rate, future_cash_flow, num_periods);
        assert_eq!(round_decimal(result), 822.7025);
    }

    #[test]
    fn future_value_basic() {
        let rate = 0.045;
        let present_cash_flow = 9000.0;
        let num_periods = 15.0;
        let result = future_value(rate, present_cash_flow, num_periods);
        assert_eq!(round_decimal(result), 17417.542);
    }

    #[test]
    fn net_present_value_basic() {
        let rate = 0.1;
        let cash_flows: [f64; 4] = [-500000.0, 200000.0, 300000.0, 200000.0];
        let result = net_present_value(rate, &cash_flows);
        assert_eq!(round_decimal(result), 80015.0263);
    }

    #[test]
    fn internal_rate_of_return_basic() {
        let cash_flows: [f64; 6] = [-5000.0, 1700.0, 1900.0, 1600.0, 1500.0, 700.0];
        let result = internal_rate_of_return(&cash_flows, None);
        assert_eq!(round_decimal(result.unwrap()), 0.1661);
    }

    #[test]
    fn internal_rate_of_return_basic_v2() {
        let cash_flows: [f64; 6] = [-2000.0, 400.0, 700.0, 500.0, 400.0, 300.0];
        let result = internal_rate_of_return(&cash_flows, None);
        assert_eq!(round_decimal(result.unwrap()), 0.0523);
    }

    #[test]
    fn payback_period_for_even_cash_flows() {
        let cash_flows: [f64; 2] = [105.0, 25.0];
        let result = payback_period(&cash_flows, 0.0);
        assert_eq!(round_decimal(result), 4.2);
    }

    #[test]
    fn payback_period_for_uneven_cash_flows() {
        let cash_flows: [f64; 6] = [-50.0, 10.0, 13.0, 16.0, 19.0, 22.0];
        let result = payback_period(&cash_flows, 5.0);
        assert_eq!(round_decimal(result), 3.4211);
    }

    #[test]
    fn return_on_investment_basic() {
        let result = return_on_investment(-55000.0, 60000.0);
        assert_eq!(round_decimal(result), 0.0909);
    }

    #[test]
    fn amortization_years() {
        let principal = 20000.0;
        let rate = 7.5;
        let num_periods = 5.0;
        let amortization_period = AmortizationPeriod::Year;
        let pay_at_beginning = false;
        let result = amortization(
            principal,
            rate,
            num_periods,
            amortization_period,
            pay_at_beginning,
        );
        assert_eq!(round_decimal(result), 400.759);
    }

    #[test]
    fn amortization_months() {
        let principal = 20000.0;
        let rate = 7.5;
        let num_periods = 60.0;
        let amortization_period = AmortizationPeriod::Month;
        let pay_at_beginning = false;
        let result = amortization(
            principal,
            rate,
            num_periods,
            amortization_period,
            pay_at_beginning,
        );
        assert_eq!(round_decimal(result), 400.759);
    }

    #[test]
    fn amortization_years_pay_at_beginning() {
        let principal = 20000.0;
        let rate = 7.5;
        let num_periods = 5.0;
        let amortization_period = AmortizationPeriod::Year;
        let pay_at_beginning = true;
        let result = amortization(
            principal,
            rate,
            num_periods,
            amortization_period,
            pay_at_beginning,
        );
        assert_eq!(round_decimal(result), 398.2698);
    }

    #[test]
    fn amortization_months_pay_at_beginning() {
        let principal = 20000.0;
        let rate = 7.5;
        let num_periods = 60.0;
        let amortization_period = AmortizationPeriod::Month;
        let pay_at_beginning = true;
        let result = amortization(
            principal,
            rate,
            num_periods,
            amortization_period,
            pay_at_beginning,
        );
        assert_eq!(round_decimal(result), 398.2698);
    }
}
