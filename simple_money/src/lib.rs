use std::collections::HashMap;
use std::ops::{Add, Sub};
use rust_decimal::Decimal;

#[derive(PartialEq, Eq, Hash)]
enum Currency {
    CAD,
    USD,
}

struct Money {
    amount: Decimal,
    currency: Currency,
    rates: HashMap<Currency,Decimal>,
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // TODO: Handle different currencies
        Self { amount: self.amount + other.amount, currency: self.currency, rates: self.rates}
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // TODO: Handle different currencies
        Self { amount: self.amount - other.amount, currency: self.currency, rates: self.rates }
    }
}