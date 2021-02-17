use rust_decimal::prelude::*;
use std::collections::HashMap;
use simple_money::*;
use rust_decimal_macros::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TaxError {
    #[error("Mismatched currencies")]
    MismatchedCurrencies,
}

#[derive(Clone, Copy, Debug)]
pub struct TaxBracket{
    min_money: Money,
    max_money: Option<Money>,
    rate: Decimal,
}

impl TaxBracket {
    fn new_tax_bracket_with_max(
        min_money: Money,
        max_money: Money,
        rate: Decimal,
    ) -> Result<TaxBracket, TaxError> {
        if min_money.currency != max_money.currency {
            Err(TaxError::MismatchedCurrencies)
        }else{
            Ok(TaxBracket{
                min_money,
                max_money: Some(max_money),
                rate,
            })
        }
    }

    pub fn new(
        min_money: Money,
        max_money: Option<Money>,
        rate: Decimal,
    ) -> Result<TaxBracket, TaxError> {
        if let Some(max_money) = max_money {
            Self::new_tax_bracket_with_max(min_money, max_money, rate)
        }else{
            Ok(TaxBracket{ min_money, max_money: None, rate})
        }
    }

    pub fn calculate_tax(&self, taxable_income: Money) -> Money {
        if taxable_income < self.min_money {
            return Money { amount: dec!(0), currency: self.min_money.currency };
        }

        if let Some(max_money) = self.max_money {
            if taxable_income >= max_money {
                return max_money * self.rate;
            }else{
                return (taxable_income - self.min_money) * self.rate;
            }
        }

        return (taxable_income - self.min_money) * self.rate;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TaxDeductionCategory {
    CapitalGains,
    LowIncome,
    CommunityFood,
    Childcare,
    PoliticalContribution,
    CoopEducation,
    ElderlyPublicTransit,
    ElderlyPropertyTax,
    TrilliumBenefit,
    NorthernOntarioEnergy,
    OntarioEnergyAndPropertyTax,
    SalesTax,
}

#[derive(Clone, Copy)]
pub struct TaxDeductionRule {
    tax_deduction_type: TaxDeductionCategory,
    max_amount: Option<Money>,
    inclusion_rate: Decimal,
}

impl TaxDeductionRule {
    pub fn apply_deduction(&self, actual_deduction: TaxDeduction) -> Money {
        if let Some(max_amount) = self.max_amount {
            if actual_deduction.money_to_deduct <= max_amount {
                return max_amount * self.inclusion_rate
            }else{
                return actual_deduction.money_to_deduct * self.inclusion_rate
            }
        }

        return actual_deduction.money_to_deduct * self.inclusion_rate;
    }
}

pub struct TaxDeduction {
    tax_deduction_type: TaxDeductionCategory,
    money_to_deduct: Money,
}

#[derive(Debug, Clone)]
pub enum TaxCalculationErrorCode {
    CouldNotFindDeduction,
}

#[derive(Clone)]
pub struct TaxRegime {
    brackets: Vec<TaxBracket>,
    deductions_map: HashMap<TaxDeductionCategory, TaxDeductionRule>,
    tax_currency: Currency,
}

impl TaxRegime {
    pub fn new(
        brackets: Vec<TaxBracket>,
        deductions_map: HashMap<TaxDeductionCategory, TaxDeductionRule>,
        currency: Currency,
    ) -> TaxRegime {
        // TODO: Validate currencies

        let mut new_brackets = brackets.clone();
        new_brackets.sort_by(|a, b| a.min_money.partial_cmp(&b.min_money).unwrap());
        return TaxRegime {
            brackets: new_brackets,
            deductions_map: deductions_map,
            tax_currency: currency,
        };
    }

    fn determine_deductions_amount(
        &self,
        deductions: Vec<TaxDeduction>,
    ) -> Result<Money, TaxCalculationErrorCode> {
        deductions
            .iter()
            .try_fold( Money { amount: dec!(0), currency: self.tax_currency } , |acc, actual_tax_deduction| {
                match self
                    .deductions_map
                    .get(&actual_tax_deduction.tax_deduction_type)
                {
                    Some(deduction_info) => {
                        let money_result = actual_tax_deduction.money_to_deduct
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
            .fold(Money { amount: dec!(0), currency: taxable_income.currency }, |acc, bracket_tax| acc + bracket_tax)
    }

    pub fn calculate_tax_with_deductions(
        &self,
        income: Money,
        deductions: Vec<TaxDeduction>,
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

    #[test]
    fn simple_example() {
        let lowest = TaxBracket {
            min_money: cad_money!(0),
            max_money: Some(cad_money!(10_000)),
            rate: dec!(0.1),
        };
        let middle = TaxBracket {
            min_money: cad_money!(10_000),
            max_money: Some(cad_money!(20_000)),
            rate: dec!(0.2),
        };
        let highest = TaxBracket {
            min_money: cad_money!(20_000),
            max_money: None,
            rate: dec!(0.3),
        };

        let regime = TaxRegime::new(vec![lowest, middle, highest], hashmap! {}, Currency::CAD);

        let over_highest_tax = regime.calculate_tax(cad_money!(25_000));
        assert_eq!(over_highest_tax, cad_money!(6_500));

        let middle_tax = regime.calculate_tax(cad_money!(15_000));
        assert_eq!(middle_tax, cad_money!(2000));

        let lowest_tax = regime.calculate_tax(cad_money!(5_000));
        assert_eq!(lowest_tax, cad_money!(500));
    }

    #[test]
    fn single_bracket_example() {
        let lowest = TaxBracket {
            min_money: cad_money!(0),
            max_money: Some(cad_money!(10_000)),
            rate: dec!(0.1),
        };

        let regime = TaxRegime::new(vec![lowest], hashmap! {}, Currency::CAD);
        let tax = regime.calculate_tax(cad_money!(10_000));

        assert_eq!(tax, cad_money!(1000));
    }

    #[test]
    fn invalid_bracket(){
        let invalid = TaxBracket::new(
            cad_money!(0), 
            Some(usd_money!(1)), 
            dec!(0.1)
        ).unwrap_err();

        assert_eq!(invalid, TaxError::MismatchedCurrencies);
    }

    #[test]
    fn deductions_example() {
        let single = TaxBracket {
            min_money: cad_money!(0),
            max_money: None,
            rate: dec!(0.1),
        };
        let capital_gains_deduction = TaxDeductionRule {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            max_amount: None,
            inclusion_rate: dec!(0.5),
        };

        let regime = TaxRegime::new(
            vec![single],
            hashmap! { TaxDeductionCategory::CapitalGains => capital_gains_deduction},
            Currency::CAD,
        );
        let actual_deductions = vec![TaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            money_to_deduct: cad_money!(5000),
        }];
        let tax = regime.calculate_tax_with_deductions(cad_money!(10_000), actual_deductions);

        match tax {
            Ok(result) => assert_eq!(result, cad_money!(750.00)),
            Err(_) => assert!(false, "Tax should not be an Err"),
        }
    }
}
