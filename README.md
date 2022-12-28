<!-- omit in toc -->
# Finance functions in Rust

[![CI](https://github.com/ddimaria/finance-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/ddimaria/finance-rs/actions/workflows/ci.yaml)

Blazingly fast financial functions in Rust.  Operations are parallelized when applicable.

- [Present Value](#present-value)
- [Future Value](#future-value)
- [Net Present Value](#net-present-value)
- [Internal Rate of Return](#internal-rate-of-return)
- [Payback Period](#payback-period)
- [Return on Investment](#return-on-investment)

<!-- omit in toc -->
## API

### Present Value

> present_value(rate: f64, future_cash_flow: f64, num_periods: f64) -> f64

```rust
let rate = 0.05;
let future_cash_flow = 1000.00;
let num_periods = 4.00;
let result = present_value(rate, future_cash_flow, num_periods);

assert_eq!(result, 822.7024747918819);
```

### Future Value

> future_value(rate: f64, present_cash_flow: f64, num_periods: f64) -> f64

```rust
let rate = 0.045;
let present_cash_flow = 9000.00;
let num_periods = 15.00;
let result = future_value(rate, present_cash_flow, num_periods);

assert_eq!(result, 17417.541987820376);
```

### Net Present Value

> net_present_value(rate: f64, cash_flows: &[f64]) -> f64

```rust
let rate = 0.1;
let cash_flows: [f64; 4] = [-500000.00, 200000.00, 300000.00, 200000.00];
let result = net_present_value(rate, &cash_flows);

assert_eq!(result, 80015.02629601792);
```


### Internal Rate of Return

> internal_rate_of_return(cash_flows: &[f64], guess: Option<f64>) -> Option<f64>

```rust
let cash_flows: [f64; 6] = [-5000.0, 1700.0, 1900.0, 1600.0, 1500.0, 700.0];
let result = internal_rate_of_return(&cash_flows, None);

assert_eq!(result.unwrap(), 0.05231449481311805);
```

### Payback Period

> payback_period(cash_flows: &[f64], num_periods: f64) -> f64

```rust
let cash_flows: [f64; 6] = [-50.0, 10.0, 13.0, 16.0, 19.0, 22.0];
let result = payback_period(&cash_flows, 5.0);

assert_eq!(result, 3.4210526315789473);
```

### Return on Investment

> return_on_investment(present_cash_flow: f64, earnings: f64) -> f64

```rust
let result = return_on_investment(-55000.0, 60000.0);

assert_eq!(result, 0.09090909090909091);
```