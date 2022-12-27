<!-- omit in toc -->
# Finance functions in Rust

[![CI](https://github.com/ddimaria/finance-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/ddimaria/finance-rs/actions/workflows/ci.yaml)

Blazingly fast financial functions in Rust.  Operations are parallelized when applicable.

- [Present Value](#present-value)
- [Future Value](#future-value)
- [Net Present Value](#net-present-value)

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