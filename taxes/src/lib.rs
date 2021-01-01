use std::{collections::HashMap};
use rusty_money::{Money};
use rust_decimal::prelude::*;

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
    pub fn calculate_tax(&self, taxable_income: Decimal) -> Decimal{
        if taxable_income < *self.min_money.amount() {
            return Decimal::zero();
        }

        if let Some(actual_max_money) = &self.max_money {
            if taxable_income >= *actual_max_money.amount() {
                return actual_max_money.amount() * self.rate;
            }else{
                return (taxable_income - self.min_money.amount()) * self.rate;
            }
        }else{
            return (taxable_income - self.min_money.amount()) * self.rate;
        }
    }
}

#[derive(Clone)]
pub struct TaxDeduction{
    tax_deduction_type: String,
    max_amount: Money,
    inclusion_rate: Decimal,
}

pub struct ActualTaxDeduction{
    tax_deduction_type: String,
    amount: Decimal,
}

pub enum TaxCalculationErrorCodes{
    CouldNotFindDeduction,
}

#[derive(Clone)]
pub struct TaxRegime {
    brackets: Vec<TaxBracket>,
    deductions_map: HashMap<String, TaxDeduction>,
}

impl TaxRegime {
    pub fn new(brackets: Vec<TaxBracket>, deductions_map: HashMap<String, TaxDeduction>) -> TaxRegime {
        let mut new_brackets = brackets.clone();
        new_brackets.sort_by(|a,b| a.min_money.partial_cmp(&b.min_money).unwrap());
        return TaxRegime { brackets: new_brackets, deductions_map: deductions_map}
    }

    fn determine_deductions_amount(&self, deductions: Vec<ActualTaxDeduction>) -> Result<Decimal, TaxCalculationErrorCodes> {
        deductions.iter().try_fold(decimal!(0, 0), |acc,actual_tax_deduction|{
            match self.deductions_map.get(&actual_tax_deduction.tax_deduction_type){
                Some(deduction_info) => {
                    Ok(acc + actual_tax_deduction.amount * deduction_info.inclusion_rate)
                },
                None => Err(TaxCalculationErrorCodes::CouldNotFindDeduction)
            }
        })
    }

    pub fn calculate_tax(&self, taxable_income: Decimal) -> Decimal{
        self.brackets.iter().map(|bracket| bracket.calculate_tax(taxable_income)).sum()
    }

    pub fn calculate_tax_with_deductions(&self, income: Decimal, deductions: Vec<ActualTaxDeduction>) -> Result<Decimal, TaxCalculationErrorCodes> {
        let deductions_amount = self.determine_deductions_amount(deductions);
        match deductions_amount {
            Ok(deductions_total) => {
                Ok(self.calculate_tax(income - deductions_total))
            },
            Err(error_code) => {
                Err(error_code)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use maplit::*;
    use super::*;
    use rusty_money::money;

    #[test]
    fn simple_example() {
        let lowest = TaxBracket { min_money: money!(0.0, "CAD"), max_money: Some(money!(10000, "CAD")), rate: decimal!(01, 1)};
        let middle = TaxBracket { min_money: money!(10000, "CAD"), max_money: Some(money!(20000,"CAD")), rate: decimal!(02, 1)};
        let highest = TaxBracket { min_money: money!(20000, "CAD"), max_money: None, rate: decimal!(03, 1)};

        let regime = TaxRegime::new(vec![lowest,middle,highest], hashmap!{});

        let over_highest_tax = regime.calculate_tax(decimal!(25000, 0));
        assert_eq!(over_highest_tax, decimal!(6500, 0));

        let middle_tax = regime.calculate_tax(decimal!(15000, 0));
        assert_eq!(middle_tax, decimal!(2000, 0));

        let lowest_tax = regime.calculate_tax(decimal!(5000, 0));
        assert_eq!(lowest_tax, decimal!(500, 0));
    }

    #[test]
    fn single_bracket_example(){
        let lowest = TaxBracket { min_money: money!(0.0, "CAD"), max_money: Some(money!(10000.0, "CAD")), rate: decimal!(01,1)};

        let regime = TaxRegime::new(vec![lowest], hashmap!{});
        let tax = regime.calculate_tax(decimal!(10000,0));

        assert_eq!(tax, decimal!(1000,0));
    }
}
