# Black-Scholes Option Pricing in Rust with Arbitrum Stylus
This repository showcases a Rust implementation of the Black-Scholes Model to calculate the European options price for an asset, specifically utilizing the Arbitrum Stylus SDK. The project is aimed at demonstrating how Rust can be used within a blockchain environment, leveraging Rust's decimal precision through the rust_decimal crate to compute options pricing efficiently.

What is Black-Scholes?
The Black-Scholes Model is a mathematical model used to calculate the theoretical price of European-style options. It is widely used in financial markets for option pricing, taking into account factors like current stock price, strike price, volatility, time to expiration, and the risk-free interest rate.

For more information on the Black-Scholes model, check out the Wikipedia page.

Features
Rust-based Implementation: Leverages the power and safety of the Rust programming language.
Precision with rust_decimal: The rust_decimal crate ensures high-precision arithmetic for financial calculations.
Arbitrum Stylus SDK: Demonstrates how to use Arbitrum's Stylus SDK to execute Rust code in an Ethereum-like environment, enabling scalability and performance.
Simple, Modular, and Scalable: Designed to be a starting point for more advanced derivatives and options pricing models.
Real-time Option Pricing: Computes the price for options based on live market data.
Setup and Installation
Prerequisites
Before getting started, you will need:

Rust installed on your machine. If you don’t have it yet, follow the instructions on the official Rust website.
Arbitrum Stylus SDK for deploying and interacting with the Rust smart contract in an Ethereum-compatible environment. Follow the Arbitrum Stylus SDK documentation.
Installation Steps
Clone this repository:

bash
Copy
Edit
git clone https://github.com/yourusername/Arbitrum-Stylus-Black-Scholes.git
cd Arbitrum-Stylus-Black-Scholes
Install dependencies: This project uses rust_decimal, an efficient and precise crate for performing financial calculations.

bash
Copy
Edit
cargo build
Deploy to Arbitrum: Follow the guidelines in the Arbitrum Stylus documentation to deploy the contract to an Arbitrum testnet or mainnet. Once deployed, you can start calling the compute method.

How It Works
The Black-Scholes Formula
The Black-Scholes formula computes the price of a European option using the following formula:

𝐶
=
𝑆
0
𝑁
(
𝑑
1
)
−
𝑋
𝑒
−
𝑟
𝑡
𝑁
(
𝑑
2
)
C=S 
0
​
 N(d 
1
​
 )−Xe 
−rt
 N(d 
2
​
 )
Where:

𝐶
C = Call option price
𝑆
0
S 
0
​
  = Current stock price
𝑋
X = Strike price of the option
𝑡
t = Time to expiration (in years)
𝑟
r = Risk-free interest rate
𝑁
(
⋅
)
N(⋅) = Cumulative distribution function of the standard normal distribution
𝑑
1
d 
1
​
  and 
𝑑
2
d 
2
​
  are intermediate calculations based on volatility and time to expiration.
This implementation in Rust computes this formula with the rust_decimal crate, which handles high precision decimals required for financial calculations.

Usage
The compute method in the contract calculates the price of the option based on the Black-Scholes model. The following parameters are used to compute the result:

stock_price: Current price of the underlying asset.
strike_price: Strike price of the option.
interest_rate: The risk-free interest rate (as a decimal).
volatility: Volatility constant for the underlying asset.
time_to_expiration: Time to expiration in years (e.g., 0.5 for 6 months).
Once the option price is computed, it returns two values:

mantissa: The resulting option price in the form of a large integer.
scaling_factor: A scaling factor to convert the mantissa into a usable decimal.
Example Usage
rust
Copy
Edit
use rust_decimal::Decimal;
use your_project::black_scholes::BlackScholes;

fn main() {
    let stock_price = Decimal::new(100, 0);  // $100
    let strike_price = Decimal::new(95, 0);  // $95
    let interest_rate = Decimal::new(5, 2);  // 5% risk-free interest rate
    let volatility = Decimal::new(20, 2);    // 20% volatility
    let time_to_expiration = Decimal::new(1, 1);  // 1 year

    let option_price = BlackScholes::compute(
        stock_price,
        strike_price,
        interest_rate,
        volatility,
        time_to_expiration,
    );

    println!("Option Price: {}", option_price);
}
Contributing
We welcome contributions! If you have improvements or fixes, feel free to fork the repository, make your changes, and submit a pull request. Be sure to follow the code style and add unit tests where applicable.

License
This project is licensed under the MIT License - see the LICENSE file for details.
