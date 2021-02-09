use rust_decimal::Decimal;

enum Currency {
    CAD,
    USD,
}

struct Money {
    amount: Decimal,
    currency: Currency,
}