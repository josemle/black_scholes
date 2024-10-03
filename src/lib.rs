//!
//! BlackScholes
//!
//! Returns BS call option formula with discount and volatility already computed.
//!
//! Note: this code is a template-only and has not been audited.
//!

// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::I256;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use stylus_sdk::{alloy_primitives::U256, console, prelude::*};

#[storage]
#[entrypoint]
struct BlackScholes;

#[public]
impl BlackScholes {
    /// Gets the number from storage.
    pub fn compute(&self) -> (I256, U256) {
        // s = current asset price
        let s = Decimal::new(100, 0);
        // k = the strike price, or price if option is exercised
        let k = Decimal::new(105, 0);
        // r = the risk-free interest rate
        let r = Decimal::new(5, 2);
        // sigma = volatility rate of the asset
        let sigma = Decimal::new(20, 2);
        // t = time to expiration (in years)
        let t = Decimal::ONE;

        let call_price = compute_all(s, k, r, sigma, t);
        let call_price_str = call_price.to_string();

        console!("Call price: {}", call_price_str);

        let mantissa = I256::try_from(call_price.mantissa()).unwrap();
        let scale = U256::from(call_price.scale());

        (mantissa, scale)
    }
}

// Currently just computes the call option price
// stock = spot price
// strike = strike price
// rate = risk-free interest rate (ie 0.03 or 0.05)
// sigma = volatility (ie 20% = 0.20)
// maturity = years to expiration (ie 1yr = 1.0)
fn compute_all(
    stock: Decimal,
    strike: Decimal,
    rate: Decimal,
    sigma: Decimal,
    maturity: Decimal,
) -> Decimal {
    let discount = (-rate * maturity).exp();
    let sqrt_maturity = maturity.sqrt().unwrap();
    let sqrt_maturity_sigma = sqrt_maturity * sigma;
    let k_discount = strike * discount;
    if sqrt_maturity_sigma > Decimal::ZERO {
        let d1 = d1(stock, strike, discount, sqrt_maturity_sigma);
        let d2 = d1 - sqrt_maturity_sigma;
        let cdf_d1 = cum_norm(d1);
        let cdf_d2 = cum_norm(d2);

        stock * cdf_d1 - k_discount * cdf_d2
    } else {
        max_or_zero(stock - strike)
    }
}

fn cum_norm(x: Decimal) -> Decimal {
    let one = Decimal::ONE;
    let two = Decimal::TWO;
    let sqrt2 = two.sqrt().unwrap();
    (x * (one / sqrt2)).erf() * dec!(0.5) + dec!(0.5)
}

fn d1(s: Decimal, k: Decimal, discount: Decimal, sqrt_maturity_sigma: Decimal) -> Decimal {
    (s / (k * discount)).ln() / sqrt_maturity_sigma + dec!(0.5) * sqrt_maturity_sigma
}

fn max_or_zero(v: Decimal) -> Decimal {
    v.max(Decimal::ZERO)
}
