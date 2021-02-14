use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};
use std::cmp::Ordering;
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Currency {
    CAD,
    USD,
}

#[derive(Debug,Error)]
pub enum MoneyError{
    #[error("Could not find exchange rate")]
    CouldNotFindExchangeRate,
    #[error("Mismatched currencies")]
    MismatchedCurrencies,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Money {
    amount: Decimal,
    currency: Currency,
}

#[derive(PartialEq, Eq, Hash)]
struct ExchangeRateQuery{
    from: Currency,
    to: Currency,
}

struct Exchange {
    rates: HashMap<ExchangeRateQuery, Decimal>,
}

impl Exchange {
    pub fn new() -> Exchange {
        return Exchange{
            rates: HashMap::new(),
        } 
    }

    pub fn set_rate(&mut self, from: Currency, to: Currency, rate: Decimal){
        let key = ExchangeRateQuery { from, to };
        let inverse_key = ExchangeRateQuery{ to, from };
        self.rates.insert(key, rate);
        self.rates.insert(inverse_key, Decimal::new(1, 0) / rate);
    }

    pub fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, MoneyError>{
        let key = ExchangeRateQuery { from, to };
        let rate = self.rates.get(&key);
        if let Some(rate) = rate {
            Ok(*rate)
        }else{
            Err(MoneyError::CouldNotFindExchangeRate)
        }
    }

    pub fn convert(&self, money: Money, currency: Currency) -> Result<Money, MoneyError> {
        let rate = self.get_rate(money.currency, currency)?;
        return Ok(money * rate);
    }
    
    pub fn add(&self, first: Money, second: Money) -> Result<Money, MoneyError> {
        if first.currency == second.currency {
            Ok(first + second)
        }else{
            self.convert(first, second.currency)
        }
    }

    pub fn sub(&self, first: Money, second: Money) -> Result<Money, MoneyError> {
        if first.currency == second.currency {
            Ok(first - second)
        }else{
            self.convert(first, second.currency)
        }
    }

    pub fn lt(&self, first: Money, second: Money) -> Result<bool, MoneyError> {
        if first.currency == second.currency {
            Ok(first < second)
        }else{
            let second_in_first_currency = self.convert(second, first.currency)?;
            Ok(first < second_in_first_currency)
        }
    }

    pub fn lte(&self, first: Money, second: Money) -> Result<bool, MoneyError> {
        if first.currency == second.currency {
            Ok(first <= second)
        }else{
            let second_in_first_currency = self.convert(second, first.currency)?;
            Ok(first <= second_in_first_currency)
        }
    }

    pub fn gte(&self, first: Money, second: Money) -> Result<bool, MoneyError> {
        if first.currency == second.currency {
            Ok(first >= second)
        }else{
            let second_in_first_currency = self.convert(second, first.currency)?;
            Ok(first >= second_in_first_currency)
        }
    }

    pub fn gt(&self, first: Money, second: Money) -> Result<bool, MoneyError> {
        if first.currency == second.currency {
            Ok(first > second)
        }else{
            let second_in_first_currency = self.convert(second, first.currency)?;
            Ok(first > second_in_first_currency)
        }
    }

    pub fn clamp(&self, input: Money, min: Money, max: Money) -> Result<Money, MoneyError>{
        if input.currency == min.currency && input.currency == max.currency {
            Ok(input.clamp(min, max))
        }else{
            let min_in_input_currency = self.convert(min, input.currency)?;
            let max_in_input_currency = self.convert(max, input.currency)?;

            Ok(input.clamp(min_in_input_currency, max_in_input_currency))
        }
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            return None;
        }

        return self.amount.partial_cmp(&other.amount)
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        }else{
            panic!("Couldn't compare Money objects")
        }
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.currency != other.currency {
            panic!("Currency mismatch");
        }

        Self { amount: self.amount + other.amount, currency: self.currency }
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.currency != other.currency {
            panic!("Currency mismatch");
        }

        Self { amount: self.amount - other.amount, currency: self.currency }
    }
}

impl Mul<Decimal> for Money {
    type Output = Self;

    fn mul(self, rhs: Decimal) -> Self::Output {
        Self { amount: self.amount * rhs, currency: self.currency }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn can_compare_usd_amount_with_greater_cad_amount(){

    }

    #[test]
    fn can_compare_usd_amount_with_less_cad_amount(){

    }

    #[test]
    fn can_compare_usd_amount_with_equal_cad_amount(){

    }

    #[test]
    fn can_clamp_with_value_less_than_range(){

    }

    #[test]
    fn can_clamp_with_value_within_range(){

    }

    #[test]
    fn can_clamp_with_value_greater_than_range(){

    }

    #[test]
    fn can_add_different_currencies_and_get_converted_result(){

    }

    #[test]
    fn can_add_same_currencies_and_get_converted_result(){

    }

    #[test]
    fn can_subtract_different_currencies_and_get_converted_result(){

    }

    #[test]
    fn can_subtract_same_currencies_and_get_converted_result(){

    }
}