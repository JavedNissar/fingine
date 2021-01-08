use rust_decimal::prelude::*;
use rusty_money::{money, Money};
use std::collections::HashMap;

macro_rules! decimal {
    ($m:expr, $e:expr) => {
        Decimal::new($m, $e)
    };
}

#[derive(Clone)]
pub struct TaxBracket {
    min_money: Money,
    max_money: Option<Money>,
    rate: Decimal,
}

impl TaxBracket {
    pub fn calculate_tax(&self, taxable_income: Money) -> Money {
        if taxable_income < self.min_money {
            return money!(0, "CAD");
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
pub struct TaxDeduction {
    tax_deduction_type: TaxDeductionCategory,
    max_amount: Option<Money>,
    inclusion_rate: Decimal,
}

impl TaxDeduction {
    pub fn apply_deduction(&self, actual_deduction: ActualTaxDeduction) -> Money {
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

pub struct ActualTaxDeduction {
    tax_deduction_type: TaxDeductionCategory,
    money_to_deduct: Money,
}

#[derive(Debug)]
pub enum TaxCalculationErrorCode {
    CouldNotFindDeduction,
}

#[derive(Clone)]
pub struct TaxRegime {
    brackets: Vec<TaxBracket>,
    deductions_map: HashMap<TaxDeductionCategory, TaxDeduction>,
}

impl TaxRegime {
    pub fn new(
        brackets: Vec<TaxBracket>,
        deductions_map: HashMap<TaxDeductionCategory, TaxDeduction>,
    ) -> TaxRegime {
        let mut new_brackets = brackets.clone();
        new_brackets.sort_by(|a, b| a.min_money.partial_cmp(&b.min_money).unwrap());
        return TaxRegime {
            brackets: new_brackets,
            deductions_map: deductions_map,
        };
    }

    fn determine_deductions_amount(
        &self,
        deductions: Vec<ActualTaxDeduction>,
    ) -> Result<Money, TaxCalculationErrorCode> {
        deductions
            .iter()
            .try_fold(money!(0, "CAD"), |acc, actual_tax_deduction| {
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

    pub fn calculate_tax(&self, taxable_income: Money) -> Money {
        self.brackets
            .iter()
            .map(|bracket| bracket.calculate_tax(taxable_income.clone()))
            .fold(money!(0, "CAD"), |acc, bracket_tax| acc + bracket_tax)
    }

    pub fn calculate_tax_with_deductions(
        &self,
        income: Money,
        deductions: Vec<ActualTaxDeduction>,
    ) -> Result<Money, TaxCalculationErrorCode> {
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
    use rusty_money::money;

    #[test]
    fn simple_example() {
        let lowest = TaxBracket {
            min_money: money!(0.0, "CAD"),
            max_money: Some(money!(10000, "CAD")),
            rate: decimal!(01, 1),
        };
        let middle = TaxBracket {
            min_money: money!(10000, "CAD"),
            max_money: Some(money!(20000, "CAD")),
            rate: decimal!(02, 1),
        };
        let highest = TaxBracket {
            min_money: money!(20000, "CAD"),
            max_money: None,
            rate: decimal!(03, 1),
        };

        let regime = TaxRegime::new(vec![lowest, middle, highest], hashmap! {});

        let over_highest_tax = regime.calculate_tax(money!(25000, "CAD"));
        assert_eq!(over_highest_tax, money!(6500, "CAD"));

        let middle_tax = regime.calculate_tax(money!(15000, "CAD"));
        assert_eq!(middle_tax, money!(2000, "CAD"));

        let lowest_tax = regime.calculate_tax(money!(5000, "CAD"));
        assert_eq!(lowest_tax, money!(500, "CAD"));
    }

    #[test]
    fn single_bracket_example() {
        let lowest = TaxBracket {
            min_money: money!(0.0, "CAD"),
            max_money: Some(money!(10000.0, "CAD")),
            rate: decimal!(01, 1),
        };

        let regime = TaxRegime::new(vec![lowest], hashmap! {});
        let tax = regime.calculate_tax(money!(10000, "CAD"));

        assert_eq!(tax, money!(1000, "CAD"));
    }

    #[test]
    fn deductions_example() {
        let single = TaxBracket {
            min_money: money!(0.0, "CAD"),
            max_money: None,
            rate: decimal!(01, 1),
        };
        let capital_gains_deduction = TaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            max_amount: None,
            inclusion_rate: decimal!(05, 1),
        };

        let regime = TaxRegime::new(
            vec![single],
            hashmap! { TaxDeductionCategory::CapitalGains => capital_gains_deduction},
        );
        let actual_deductions = vec![ActualTaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            money_to_deduct: money!(5000, "CAD"),
        }];
        let tax = regime.calculate_tax_with_deductions(money!(10000, "CAD"), actual_deductions);

        match tax {
            Ok(result) => assert_eq!(result, money!(500, "CAD")),
            Err(_) => assert!(false, "Tax should not be an Err"),
        }
    }
}
