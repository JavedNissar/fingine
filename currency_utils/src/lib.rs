use anyhow::{ensure, Error, Result};
use rusty_money::{Currency, Exchange, Money};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("Could not find exchange rate")]
    CouldNotFindExchangeRate,
    #[error("Could not convert money to currency")]
    CouldNotConvert,
}

pub trait CurrencyIndependentClamp {
    fn clamp(&self, min_money: Money, max_money: Money, exchange: Exchange) -> Result<Money>;
}

pub trait CurrencyIndependentComparison {
    fn lt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn lte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn gt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn gte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
    fn eq(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode>;
}

pub trait CurrencyIndependentAdd {
    fn add(
        &self,
        other: Money,
        output_currency: Currency,
        exchange: Exchange,
    ) -> Result<Money, ErrorCode>;
}

pub trait CurrencyIndependentSub {
    fn sub(
        &self,
        other: Money,
        output_currency: Currency,
        exchange: Exchange,
    ) -> Result<Money, ErrorCode>;
}

impl CurrencyIndependentComparison for Money {
    fn lt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let exchange_rate_pair = exchange.get_rate(self.currency(), other.currency());
        if let Some(exchange_rate_pair) = exchange_rate_pair {
            let cur_money = exchange_rate_pair.convert(self.clone());
            if let Ok(cur_money) = cur_money {
                Ok(cur_money.amount() < other.amount())
            } else {
                Err(ErrorCode::CouldNotFindExchangeRate)
            }
        } else {
            Err(ErrorCode::CouldNotFindExchangeRate)
        }
    }

    fn lte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let exchange_rate_pair = exchange.get_rate(self.currency(), other.currency());
        if let Some(exchange_rate_pair) = exchange_rate_pair {
            let cur_money = exchange_rate_pair.convert(self.clone());
            if let Ok(cur_money) = cur_money {
                Ok(cur_money.amount() <= other.amount())
            } else {
                Err(ErrorCode::CouldNotFindExchangeRate)
            }
        } else {
            Err(ErrorCode::CouldNotFindExchangeRate)
        }
    }

    fn gt(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let exchange_rate_pair = exchange.get_rate(self.currency(), other.currency());
        if let Some(exchange_rate_pair) = exchange_rate_pair {
            let cur_money = exchange_rate_pair.convert(self.clone());
            if let Ok(cur_money) = cur_money {
                Ok(cur_money.amount() > other.amount())
            } else {
                Err(ErrorCode::CouldNotFindExchangeRate)
            }
        } else {
            Err(ErrorCode::CouldNotFindExchangeRate)
        }
    }

    fn gte(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let exchange_rate_pair = exchange.get_rate(self.currency(), other.currency());
        if let Some(exchange_rate_pair) = exchange_rate_pair {
            let cur_money = exchange_rate_pair.convert(self.clone());
            if let Ok(cur_money) = cur_money {
                Ok(cur_money.amount() >= other.amount())
            } else {
                Err(ErrorCode::CouldNotFindExchangeRate)
            }
        } else {
            Err(ErrorCode::CouldNotFindExchangeRate)
        }
    }

    fn eq(&self, other: Money, exchange: Exchange) -> Result<bool, ErrorCode> {
        let exchange_rate_pair = exchange.get_rate(self.currency(), other.currency());
        if let Some(exchange_rate_pair) = exchange_rate_pair {
            let cur_money = exchange_rate_pair.convert(self.clone());
            if let Ok(cur_money) = cur_money {
                Ok(cur_money.amount() == other.amount())
            } else {
                Err(ErrorCode::CouldNotFindExchangeRate)
            }
        } else {
            Err(ErrorCode::CouldNotFindExchangeRate)
        }
    }
}

impl CurrencyIndependentClamp for Money {
    fn clamp(&self, min_money: Money, max_money: Money, exchange: Exchange) -> Result<Money> {
        let currency = self.currency();
        let min_currency = min_money.currency();
        let max_currency = max_money.currency();

        let exchange_rate_pair = (
            exchange.get_rate(currency, min_currency),
            exchange.get_rate(currency, max_currency),
        );

        match exchange_rate_pair {
            (Some(min_rate), Some(max_rate)) => {
                let cur_to_min_currency_result = min_rate.convert(self);
                if let Ok(cur_in_min_currency) = cur_to_min_currency_result {
                } else {
                }
            }
            (Some(min_rate), None) => {}
            (None, Some(max_rate)) => {}
            (None, None) => Err(ErrorCode::CouldNotFindExchangeRate),
        }
    }
}

impl CurrencyIndependentAdd for Money {}

impl CurrencyIndependentSub for Money {}
