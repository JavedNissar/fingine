use anyhow::{Error, Result};
use rusty_money::{Currency, Exchange, Money, MoneyError};
use thiserror::Error;

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
    fn from(moneyError: MoneyError) -> Self {
        match moneyError {
            MoneyError::InvalidAmount => ErrorCode::InvalidAmount,
            // Whenever we have an invalid currency, that's usually because an arithmetic operation on money involved multiple currencies
            MoneyError::InvalidCurrency => ErrorCode::CouldNotMatchCurrencies,
            MoneyError::InvalidRatio => ErrorCode::InvalidRatio,
        }
    }
}


fn convert(exchange: &Exchange, money: &Money, currency: &'static Currency) -> Result<Money, ErrorCode> {
    let exchange_rate_pair = exchange.get_rate(money.currency(), currency);

    if let Some(exchange_rate_pair) = exchange_rate_pair {
        let cur_money = exchange_rate_pair.convert(*money)?;
        Ok(cur_money)
    } else{
        Err(ErrorCode::CouldNotFindExchangeRate)
    }
}

pub trait CurrencyIndependentClamp {
    fn clamp(&self, min_money: Money, max_money: Money, exchange: Exchange) -> Result<Money, ErrorCode>;
}

pub trait CurrencyIndependentComparison {
    fn currency_independent_lt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn currency_independent_lte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn currency_independent_gt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn currency_independent_gte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn currency_independent_eq(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
}

pub trait CurrencyIndependentAdd {
    fn add(
        &self,
        other: Money,
        output_currency: &'static Currency,
        exchange: Exchange,
    ) -> Result<Money, ErrorCode>;
}

pub trait CurrencyIndependentSub {
    fn sub(
        &self,
        other: Money,
        output_currency: &'static Currency,
        exchange: Exchange,
    ) -> Result<Money, ErrorCode>;
}

impl CurrencyIndependentComparison for Money {
    fn currency_independent_lt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let cur_money = convert(&exchange, self, other.currency())?;
        Ok(cur_money.amount() < other.amount())
    }

    fn currency_independent_lte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let cur_money = convert(&exchange, self, other.currency())?;
        Ok(cur_money.amount() <= other.amount())
    }

    fn currency_independent_gt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let cur_money = convert(&exchange, self, other.currency())?;
        Ok(cur_money.amount() > other.amount())
    }

    fn currency_independent_gte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let cur_money = convert(&exchange, self, other.currency())?;
        Ok(cur_money.amount() >= other.amount())
    }

    fn currency_independent_eq(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let cur_money = convert(&exchange, self, other.currency())?;
        Ok(cur_money.amount() == other.amount())
    }
}

pub enum PositionRelativeToRange{
    BeforeRange,
    WithinRange,
    AfterRange,
}

fn determine_relative_position_of_money_relative_to_range(money_to_consider: Money, min_money: Money, max_money: Money, exchange: Exchange) -> Result<PositionRelativeToRange, ErrorCode> {
    let less_than_min_result = money_to_consider.currency_independent_lt(min_money, exchange)?;
    let less_than_max_result = money_to_consider.currency_independent_lt(max_money, exchange)?;
    let greater_than_or_equal_to_max_result = money_to_consider.currency_independent_gte(max_money, exchange)?;

    if less_than_min_result {
        Ok(PositionRelativeToRange::BeforeRange)
    }else if less_than_max_result {
        Ok(PositionRelativeToRange::WithinRange)
    }else{
        Ok(PositionRelativeToRange::AfterRange)
    }
}

impl CurrencyIndependentClamp for Money {
    fn clamp(&self, min_money: Money, max_money: Money, exchange: Exchange) -> Result<Money, ErrorCode> {
        let currency = self.currency();
        let min_currency = min_money.currency();
        let max_currency = max_money.currency();

        let relative_to_range = determine_relative_position_of_money_relative_to_range(*self, min_money, max_money, exchange)?;
        match relative_to_range {
            PositionRelativeToRange::BeforeRange => convert(&exchange, &min_money, self.currency()),
            PositionRelativeToRange::WithinRange => Ok(*self),
            PositionRelativeToRange::AfterRange => convert(&exchange, &max_money, self.currency()),
        }
    }
}

impl CurrencyIndependentAdd for Money {
    fn add(&self, other: Money, output_currency: &'static Currency, exchange: Exchange) -> Result<Money, ErrorCode> {
        let converted_self = convert(&exchange, &self, output_currency)?;
        let converted_other = convert(&exchange, &other, output_currency)?;
        
        Ok(converted_self + converted_other)
    }
}

impl CurrencyIndependentSub for Money {
    fn sub(&self, other: Money, output_currency: &'static Currency, exchange: Exchange) -> Result<Money, ErrorCode> {
        let converted_self = convert(&exchange, &self, output_currency)?;
        let converted_other = convert(&exchange,&other, output_currency)?;

        Ok(converted_self - converted_other)
    }
}
