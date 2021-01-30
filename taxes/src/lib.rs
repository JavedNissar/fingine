use rust_decimal::prelude::*;
use rusty_money::{Money, FormattableCurrency, define_currency_set};
use std::collections::HashMap;
use currency_utils;

macro_rules! decimal {
    ($m:expr, $e:expr) => {
        Decimal::new($m, $e)
    };
}

#[derive(Clone)]
pub struct TaxBracket<'a, T: FormattableCurrency> {
    min_money: Money<'a, T>,
    max_money: Option<Money<'a, T>>,
    rate: Decimal,
    tax_currency: &'a T,
}

impl<'a, T: FormattableCurrency> TaxBracket<'a, T> {
    pub fn calculate_tax(&self, taxable_income: Money<'a, T>) -> Money<'a, T> {
        if taxable_income < self.min_money {
            return Money::from_minor(0, self.tax_currency);
        }

        if let Some(actual_max_money) = &self.max_money {
            let actual_max_money_clone = actual_max_money.clone();
            if taxable_income >= actual_max_money_clone {
                return actual_max_money_clone * self.rate;
            } else {
                return (taxable_income - self.min_money.clone()) * self.rate;
            }
        } else {
            return (taxable_income - self.min_money.clone()) * self.rate;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TaxDeductionCategory {
    CapitalGains,
}

#[derive(Clone)]
pub struct TaxDeduction<'a, T: FormattableCurrency> {
    tax_deduction_type: TaxDeductionCategory,
    max_amount: Option<Money<'a, T>>,
    inclusion_rate: Decimal,
}

impl<'a, T: FormattableCurrency> TaxDeduction<'a, T> {
    pub fn apply_deduction(&self, actual_deduction: ActualTaxDeduction<'a, T>) -> Money<'a, T> {
        match &self.max_amount {
            Some(inner_max_amount) => {
                if actual_deduction.money_to_deduct.amount() <= inner_max_amount.amount() {
                    inner_max_amount.clone() * self.inclusion_rate
                } else {
                    actual_deduction.money_to_deduct * self.inclusion_rate
                }
            }
            None => actual_deduction.money_to_deduct * self.inclusion_rate,
        }
    }
}

pub struct ActualTaxDeduction<'a, T: FormattableCurrency> {
    tax_deduction_type: TaxDeductionCategory,
    money_to_deduct: Money<'a, T>,
}

#[derive(Debug)]
pub enum TaxCalculationErrorCode {
    CouldNotFindDeduction,
}

#[derive(Clone)]
pub struct TaxRegime<'a, T: FormattableCurrency> {
    brackets: Vec<TaxBracket<'a, T>>,
    deductions_map: HashMap<TaxDeductionCategory, TaxDeduction<'a, T>>,
    tax_currency: &'a T,
}

impl<'a, T: FormattableCurrency> TaxRegime<'a, T> {
    pub fn new(
        brackets: Vec<TaxBracket<'a, T>>,
        deductions_map: HashMap<TaxDeductionCategory, TaxDeduction<'a, T>>,
        tax_currency: &'a T,
    ) -> TaxRegime<'a, T> {
        let mut new_brackets = brackets.clone();
        new_brackets.sort_by(|a, b| a.min_money.partial_cmp(&b.min_money).unwrap());
        return TaxRegime {
            brackets: new_brackets,
            deductions_map: deductions_map,
            tax_currency: tax_currency,
        };
    }

    fn determine_deductions_amount(
        &self,
        deductions: Vec<ActualTaxDeduction<'a, T>>,
    ) -> Result<Money<'a, T>, TaxCalculationErrorCode> {
        deductions
            .iter()
            .try_fold(Money::from_minor(0, self.tax_currency), |acc, actual_tax_deduction| {
                match self
                    .deductions_map
                    .get(&actual_tax_deduction.tax_deduction_type)
                {
                    Some(deduction_info) => {
                        let money_result = actual_tax_deduction.money_to_deduct.clone()
                            * deduction_info.inclusion_rate
                            + acc;
                        Ok(money_result)
                    }
                    None => Err(TaxCalculationErrorCode::CouldNotFindDeduction),
                }
            })
    }

    pub fn calculate_tax(&self, taxable_income: Money<'a, T>) -> Money<'a, T> {
        self.brackets
            .iter()
            .map(|bracket| bracket.calculate_tax(taxable_income.clone()))
            .fold(Money::from_minor(0, taxable_income.currency()), |acc, bracket_tax| acc + bracket_tax)
    }

    pub fn calculate_tax_with_deductions(
        &self,
        income: Money<'a, T>,
        deductions: Vec<ActualTaxDeduction<'a, T>>,
    ) -> Result<Money<'a, T>, TaxCalculationErrorCode> {
        let deductions_amount = self.determine_deductions_amount(deductions);
        match deductions_amount {
            Ok(deductions_total) => Ok(self.calculate_tax(income - deductions_total)),
            Err(error_code) => Err(error_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::*;

    define_currency_set!(
        test {
            CAD: {
                code: "CAD",
                exponent: 2,
                locale: EnUs,
                minor_units: 100,
                name: "CAD",
                symbol: "$",
                symbol_first: true,
            }
        }
    );

    #[test]
    fn simple_example() {
        let cad = test::find("CAD").unwrap();        

        let lowest = TaxBracket {
            min_money: Money::from_minor(0, cad), 
            max_money: Some(Money::from_minor(10_000_00, cad)),
            rate: decimal!(0_1, 1),
            tax_currency: cad,
        };
        let middle = TaxBracket {
            min_money: Money::from_minor(10_000, cad),
            max_money: Some(Money::from_minor(20_000, cad)),
            rate: decimal!(0_2, 1),
            tax_currency: cad,
        };
        let highest = TaxBracket {
            min_money: Money::from_minor(20_000, cad),
            max_money: None,
            rate: decimal!(0_3, 1),
            tax_currency: cad,
        };

        let regime = TaxRegime::new(vec![lowest, middle, highest], hashmap! {}, cad);

        let over_highest_tax = regime.calculate_tax(Money::from_minor(25_000, cad));
        assert_eq!(over_highest_tax, Money::from_minor(6_500, cad));

        let middle_tax = regime.calculate_tax(Money::from_minor(15_000, cad));
        assert_eq!(middle_tax, Money::from_minor(2000, cad));

        let lowest_tax = regime.calculate_tax(Money::from_minor(5_000, cad));
        assert_eq!(lowest_tax, Money::from_minor(500, cad));
    }

    #[test]
    fn single_bracket_example() {
        let cad = test::find("CAD").unwrap();        

        let lowest = TaxBracket {
            min_money: Money::from_minor(0_0, cad),
            max_money: Some(Money::from_minor(10_000_00, cad)),
            rate: decimal!(0_1, 1),
            tax_currency: cad,
        };

        let regime = TaxRegime::new(vec![lowest], hashmap! {}, cad);
        let tax = regime.calculate_tax(Money::from_minor(10_000_00, cad)); 

        assert_eq!(tax, Money::from_minor(1_000_00, cad));
    }

    #[test]
    fn deductions_example() {
        let cad = test::find("CAD").unwrap();        

        let single = TaxBracket {
            min_money: Money::from_minor(0_0, cad),
            max_money: None,
            rate: decimal!(01, 1),
            tax_currency: cad,
        };
        let capital_gains_deduction = TaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            max_amount: None,
            inclusion_rate: decimal!(05, 1),
        };

        let regime = TaxRegime::new(
            vec![single],
            hashmap! { TaxDeductionCategory::CapitalGains => capital_gains_deduction},
            cad,
        );
        let actual_deductions = vec![ActualTaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            money_to_deduct: Money::from_minor(5_000_00, cad),
        }];
        let tax = regime.calculate_tax_with_deductions(Money::from_minor(10_000_00, cad), actual_deductions);

        match tax {
            Ok(result) => assert_eq!(result, Money::from_minor(500_00, cad)),
            Err(_) => assert!(false, "Tax should not be an Err"),
        }
    }
}
