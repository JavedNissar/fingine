use anyhow::Result;
use rusty_money::{FormattableCurrency, Exchange, Money, MoneyError, ExchangeRate};
use rust_decimal::*;
use thiserror::Error;
use rust_decimal_macros::*;

macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr, $eps: expr) => {
        assert_eq!(approximately_equal($lhs, $rhs, $eps), true)
    };
}

#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("Could not find exchange rate")]
    CouldNotFindExchangeRate,
    #[error("Could not convert money to currency")]
    CouldNotConvert,
    #[error("Currencies did not match for conversion")]
    CouldNotMatchCurrencies,
    #[error("Amount of money was not sensible")]
    InvalidAmount,
    #[error("Ratio was invalid")]
    InvalidRatio,
}

impl From<MoneyError> for ErrorCode {
    fn from(money_error: MoneyError) -> Self {
        match money_error {
            MoneyError::InvalidAmount => ErrorCode::InvalidAmount,
            // Whenever we have an invalid currency, that's usually because an arithmetic operation on money involved multiple currencies
            MoneyError::InvalidCurrency => ErrorCode::CouldNotMatchCurrencies,
            MoneyError::InvalidRatio => ErrorCode::InvalidRatio,
        }
    }
}

fn convert<'a, T: FormattableCurrency>(exchange: &Exchange<'a, T>, money: &Money<'a, T>, currency: &'a T) -> Result<Money<'a, T>, ErrorCode> {
    let exchange_rate_pair = exchange.get_rate(money.currency(), currency);

    if let Some(exchange_rate_pair) = exchange_rate_pair {
        let cur_money = exchange_rate_pair.convert(money.clone())?;
        Ok(cur_money)
    } else{
        Err(ErrorCode::CouldNotFindExchangeRate)
    }
}

pub trait CurrencyIndependentClamp<'a, T: FormattableCurrency> {
    fn currency_independent_clamp(&self, min_money: &Money<'a, T>, max_money: &Money<'a, T>, exchange: &Exchange<'a, T>, output_currency: &'a T) -> Result<Money<'a, T>, ErrorCode>;
}

pub trait CurrencyIndependentComparison<'a, T: FormattableCurrency> {
    fn currency_independent_lt(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode>;
    fn currency_independent_lte(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode>;
    fn currency_independent_gt(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode>;
    fn currency_independent_gte(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode>;
    fn currency_independent_eq(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode>;
}

pub trait CurrencyIndependentAdd<'a, T: FormattableCurrency> {
    fn currency_independent_add(
        &self,
        other: &Money<'a, T>,
        output_currency: &'a T,
        exchange: &Exchange<'a, T>,
    ) -> Result<Money<'a, T>, ErrorCode>;
}

pub trait CurrencyIndependentSub<'a, T: FormattableCurrency> {
    fn currency_independent_sub(
        &self,
        other: &Money<'a, T>,
        output_currency: &'a T,
        exchange: &Exchange<'a, T>,
    ) -> Result<Money<'a, T>, ErrorCode>;
}

impl<'a, T: FormattableCurrency> CurrencyIndependentComparison<'a, T> for Money<'a, T> {
    fn currency_independent_lt(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode> {
        let cur_money = convert(exchange, self, other.currency())?;
        Ok(cur_money.amount() < other.amount())
    }

    fn currency_independent_lte(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode> {
        let cur_money = convert(exchange, self, other.currency())?;
        Ok(cur_money.amount() <= other.amount())
    }

    fn currency_independent_gt(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode> {
        let cur_money = convert(exchange, self, other.currency())?;
        Ok(cur_money.amount() > other.amount())
    }

    fn currency_independent_gte(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode> {
        let cur_money = convert(exchange, self, other.currency())?;
        Ok(cur_money.amount() >= other.amount())
    }

    fn currency_independent_eq(&self, other: &Money<'a, T>, exchange: &Exchange<'a, T>) -> Result<bool, ErrorCode> {
        let cur_money = convert(exchange, self, other.currency())?;
        Ok(cur_money.amount() == other.amount())
    }
}

pub enum PositionRelativeToRange{
    BeforeRange,
    WithinRange,
    AfterRange,
}

fn determine_relative_position_of_money_relative_to_range<'a, T:FormattableCurrency>(
    money_to_consider: &Money<'a, T>, 
    min_money: &Money<'a, T>, 
    max_money: &Money<'a, T>, 
    exchange: &Exchange<'a, T>) 
    -> Result<PositionRelativeToRange, ErrorCode> 
{
    let less_than_min_result = money_to_consider.currency_independent_lt(min_money, exchange)?;
    let less_than_max_result = money_to_consider.currency_independent_lt(max_money, exchange)?;

    if less_than_min_result {
        Ok(PositionRelativeToRange::BeforeRange)
    }else if less_than_max_result {
        Ok(PositionRelativeToRange::WithinRange)
    }else{
        Ok(PositionRelativeToRange::AfterRange)
    }
}

impl<'a, T: FormattableCurrency> CurrencyIndependentClamp<'a, T> for Money<'a, T>{
    fn currency_independent_clamp(&self, min_money: &Money<'a, T>, max_money: &Money<'a, T>, exchange: &Exchange<'a, T>, output_currency: &'a T) -> Result<Money<'a, T>, ErrorCode> {
        let relative_to_range = determine_relative_position_of_money_relative_to_range(self, min_money, max_money, exchange)?;
        match relative_to_range {
            PositionRelativeToRange::BeforeRange => convert(exchange, &min_money, output_currency),
            PositionRelativeToRange::WithinRange => convert(exchange, &self, output_currency),
            PositionRelativeToRange::AfterRange => convert(exchange, &max_money, output_currency),
        }
    }
}

impl<'a, T: FormattableCurrency> CurrencyIndependentAdd<'a, T> for Money<'a, T>{
    fn currency_independent_add(&self, other: &Money<'a, T>, output_currency: &'a T, exchange: &Exchange<'a, T>) -> Result<Money<'a, T>, ErrorCode> {
        let converted_self = convert(exchange, &self, output_currency)?;
        let converted_other = convert(exchange, &other, output_currency)?;
        
        Ok(converted_self + converted_other)
    }
}

impl<'a, T: FormattableCurrency> CurrencyIndependentSub<'a, T> for Money<'a, T>{
    fn currency_independent_sub(&self, other: &Money<'a, T>, output_currency: &'a T, exchange: &Exchange<'a, T>) -> Result<Money<'a, T>, ErrorCode> {
        let converted_self = convert(exchange, &self, output_currency)?;
        let converted_other = convert(exchange,&other, output_currency)?;

        Ok(converted_self - converted_other)
    }
}

fn approximately_equal(orig: &Decimal, other: &Decimal, epsilon: &Decimal) -> bool{
    (orig - other < *epsilon) || (other - orig < *epsilon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_money::define_currency_set;
    use rust_decimal_macros::*;

    define_currency_set!(
        test {
            USD: {
                code: "USD",
                exponent: 2,
                locale: EnUs,
                minor_units: 100,
                name: "USD",
                symbol: "$",
                symbol_first: true,
            },
            GBP : {
                code: "GBP",
                exponent: 2,
                locale: EnUs,
                minor_units: 1,
                name: "British Pound",
                symbol: "£",
                symbol_first: true,
            },
            EUR : {
                code: "EUR",
                exponent: 2,
                locale: EnEu,
                minor_units: 1,
                name: "Euro",
                symbol: "€",
                symbol_first: true,
            }
        }
    );

    #[test]
    fn can_compare_usd_amount_with_greater_gbp_amount(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let gbp_usd_rate = ExchangeRate::new(gbp, usd, dec!(1) / dec!(0.7)).unwrap();
        let mut exchange = Exchange::new(); 

        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&gbp_usd_rate);
        
        let usd_amount = Money::from_minor(2_00, test::USD);
        let gbp_amount = Money::from_minor(3_00, test::GBP);

        assert_eq!(usd_amount.currency_independent_lt(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_lte(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_gt(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_gte(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_eq(&gbp_amount, &exchange).unwrap(), false);

        assert_eq!(gbp_amount.currency_independent_lt(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_lte(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_gt(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_gte(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_eq(&usd_amount, &exchange).unwrap(), false);
    }

    #[test]
    fn can_compare_usd_amount_with_lesser_gbp_amount(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let mut exchange = Exchange::new(); 

        exchange.set_rate(&usd_gbp_rate);
        
        let usd_amount = Money::from_minor(5_00, test::USD);
        let gbp_amount = Money::from_minor(1_00, test::GBP);

        assert_eq!(usd_amount.currency_independent_lt(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_lte(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_gt(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_gte(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_eq(&gbp_amount, &exchange).unwrap(), false);

        assert_eq!(gbp_amount.currency_independent_lt(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_lte(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_gt(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_gte(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_eq(&usd_amount, &exchange).unwrap(), false);
    }

    #[test]
    fn can_compare_usd_amount_with_equal_gbp_amount(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let mut exchange = Exchange::new(); 

        exchange.set_rate(&usd_gbp_rate);
        
        let usd_amount = Money::from_minor(10_00, test::USD);
        let gbp_amount = Money::from_minor(7_00, test::GBP);

        assert_eq!(usd_amount.currency_independent_lt(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_lte(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_gt(&gbp_amount, &exchange).unwrap(), false);
        assert_eq!(usd_amount.currency_independent_gte(&gbp_amount, &exchange).unwrap(), true);
        assert_eq!(usd_amount.currency_independent_eq(&gbp_amount, &exchange).unwrap(), true);

        assert_eq!(gbp_amount.currency_independent_lt(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_lte(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_gt(&usd_amount, &exchange).unwrap(), false);
        assert_eq!(gbp_amount.currency_independent_gte(&usd_amount, &exchange).unwrap(), true);
        assert_eq!(gbp_amount.currency_independent_eq(&usd_amount, &exchange).unwrap(), true);
    }

    #[test]
    fn can_clamp_with_value_less_than_range(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let usd_amount = Money::from_minor(8_00, test::USD);
        let min_in_gbp = Money::from_minor(6_00, test::GBP);
        let max_in_eur = Money::from_minor(9_00, test::EUR);

        let clamped = usd_amount.currency_independent_clamp(&min_in_gbp, &max_in_eur, &exchange, test::USD).unwrap();

        assert_eq!(clamped.amount(), min_in_gbp.amount());
    }

    #[test]
    fn can_clamp_with_value_within_range(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let usd_amount = Money::from_minor(9_00, test::USD);
        let min_in_gbp = Money::from_minor(6_00, test::GBP);
        let max_in_eur = Money::from_minor(9_00, test::EUR);

        let clamped = usd_amount.currency_independent_clamp(&min_in_gbp, &max_in_eur, &exchange, test::USD).unwrap();
        let expected_usd_amount = Money::from_minor(8_57, test::USD);

        assert_eq!(clamped.amount(), expected_usd_amount.amount());
    }

    #[test]
    fn can_clamp_with_value_greater_than_range(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let usd_amount = Money::from_minor(12_00, test::USD);
        let min_in_gbp = Money::from_minor(6_00, test::GBP);
        let max_in_eur = Money::from_minor(9_00, test::EUR);

        let clamped = usd_amount.currency_independent_clamp(&min_in_gbp, &max_in_eur, &exchange, test::USD).unwrap();
        let expected_usd_amount = Money::from_minor(11_25, test::USD);

        assert_eq!(clamped.amount(), expected_usd_amount.amount());
    }

    #[test]
    fn can_add_different_currencies_and_get_converted_result(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let gbp_amount = Money::from_minor(10_00, test::GBP);
        let eur_amount = Money::from_minor(10_00, test::EUR);
        let summed_usd_amount = Money::from_minor(26_79, test::USD);

        let sum_of_different_currencies = gbp_amount.currency_independent_add(
            &eur_amount, 
            test::USD, 
            &exchange).unwrap();

        
        assert_approx_eq!(sum_of_different_currencies.amount(), summed_usd_amount.amount(), &dec!(0.1));
    }

    #[test]
    fn can_add_same_currencies_and_get_converted_result(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let first_gbp_amount = Money::from_minor(10_00, test::GBP);
        let second_gbp_amount= Money::from_minor(10_00, test::EUR);
        let summed_usd_amount = Money::from_minor(26_79, test::USD);

        let sum_of_different_currencies = first_gbp_amount.currency_independent_add(
            &second_gbp_amount, 
            test::USD, 
            &exchange).unwrap();

        assert_eq!(sum_of_different_currencies.amount(), summed_usd_amount.amount());
    }

    #[test]
    fn can_subtract_different_currencies_and_get_converted_result(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let gbp_amount = Money::from_minor(10_00, test::GBP);
        let eur_amount = Money::from_minor(10_00, test::EUR);
        let usd_amount = Money::from_minor(1_79, test::USD);

        let difference = gbp_amount.currency_independent_sub(
            &eur_amount, 
            test::USD, 
            &exchange).unwrap();

        assert_eq!(difference.amount(), usd_amount.amount());
    }

    #[test]
    fn can_subtract_same_currencies_and_get_converted_result(){
        let usd = test::find("USD").unwrap();
        let gbp = test::find("GBP").unwrap();
        let eur = test::find("EUR").unwrap();

        let usd_gbp_rate = ExchangeRate::new(usd, gbp, dec!(0.7)).unwrap();
        let usd_eur_rate = ExchangeRate::new(usd, eur, dec!(0.8)).unwrap();

        let mut exchange = Exchange::new();
        exchange.set_rate(&usd_gbp_rate);
        exchange.set_rate(&usd_eur_rate);

        let first_gbp_amount = Money::from_minor(10_00, test::GBP);
        let second_gbp_amount= Money::from_minor(9_00, test::GBP);
        let usd_amount = Money::from_minor(0_70, test::USD);

        let difference= first_gbp_amount.currency_independent_sub(
            &second_gbp_amount, 
            test::USD, 
            &exchange).unwrap();

        assert_eq!(difference.amount(), usd_amount.amount());
    }
}