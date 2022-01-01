# About

Fingine is a personal financial simulation engine that is intended to enable folks to write scripts that allow them to simulate their finances in various scenarios.

# Principles

1. Emphasize geographic and temporal flexibility with all features 
2. Provide a reasonably easy environment to work in for scripting financial planning scenarios
3. Emphasize performance in order to enable folks to iterate on their scripts quickly

# Components

- `simple_money`: a crate that encodes the concept of money. This should be available independently of `fingine`.
- `income_tax_engine`: a crate that will calculate taxes across a progressive income tax system and provide hooks for deductions and credits. This should be available independently of `fingine`.
- `financial_primitives`: Model a variety of financial primitives such as loans, stock options, stocks, mortgages, bonds, annuities, crypto, commodities, and real estate. This should further include financial events such as receiving payments, spending, investment growth, and downturns. 
- `fingine`: an application that exposes various primitives to scripts which are intended to encode scenarios in personal finance.

# Roadmap

Goals for 2022:
* Complete implementation and release of `simple_money` to allow for the modeling of a majority of the world's currencies 
* Complete implementation and release of `income_tax_engine` to allow for the modeling of a majority of the world's tax engines. This will require deferring some of the modelling onto users; for example, I expect the concept of capital gains will not be encoded in the `income_tax_engine` as what that entails is very country dependent.
* Complete implementation of `financial_primitives`
* Choose scripting language for `fingine` and plan how to expose underlying components to scripts.
