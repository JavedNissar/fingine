# About

Finsim is a Rust library that intends to provide personal finance simulation capabilities within the Canadian and American regulatory environments.

# Principles

1. Emphasize geographic and temporal flexibility with all features with the caveat that this should only work in Canada and the US
2. Emphasize modularization to enable finsim's components to be used as individual libraries

# Components

- `simple_money`: a crate that encodes the concept of money
- `tax_engine`: a crate that will calculate taxes accounting for deductions and credits across Canada and the US
- `finsim`: a crate that will encode the concept of accounts and assets as well as an event system that can be used to modify accounts and assets 

# Roadmap

A useful framing I uncovered while thinking about what I would like this project to achieve is what questions do I hope to help people (and myself) answer 
this year. For 2021, I would like to help folks answer the following question:

* When given two job offers in Canada, how can someone figure out which one is better from the perspective of increasing their net worth over the amount of time they expect to be in that job?