# Arbitrum Stylus Black Scholes

An implementation of Black-Scholes in Rust, built with the Arbitrum Stylus SDK. This is a demonstration of using the `rust_decimal` crate to compute the European options price for an asset (listed as "stock" price in the code) given a current price, a strike price, a risk-free interest rate, a volatility constant, and the time to expiration.

The `compute` method on the sample contract will compute the price according to the model and will return a mantissa and a scaling factor for producing the price as a decimal.

See [Black-Scholes (Wikipedia)](https://en.wikipedia.org/wiki/Black%E2%80%93Scholes_equation) for more info.
