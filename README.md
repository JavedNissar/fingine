# About

Finsim is a Rust library that intends to provide personal finance simulation capabilities within the Canadian and American regulatory environments.

# Principles

1. Focus on features and situations useful to those who are in the process of building wealth
2. Emphasize geographic and temporal flexibility with all features
3. Emphasize modularization to enable finsim's components to be used as individual libraries

# Roadmap

A useful framing I uncovered while thinking about what I would like this project to achieve is what questions do I hope to help people (and myself) answer 
this year. For 2021, I would like to help folks answer the following questions:

* If someone owns or runs a startup, should they use their RRSP contribution room now or when their startup IPOs?
* When given two job offers, how does someone  decide which one?

From considering what to build out to answer those questions, I came up with the following roadmap:

1. A tax engine that accounts for common jurisdiction for software engineers in Canada: Quebec, Ontario, BC, and Alberta.
2. A system for managing a portfolio with registered accounts (ex. TFSA/RRSP), calculating it's growth over a time period, accounting for inflation, modelling asset allocations and the different returns one can expect out of different assets.
3. Events, one's financial life is dictated by financial events with examples being income events and expense events. It will probably be useful to model these events as streams in some cases (ex. salary)
4. Modelling equity packages with exercise costs (for options). a vesting schedule, fair market value, and expected stock growth.
5. Modelling investment strategies, given a portfolio and an event, decide what to do.