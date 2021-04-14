use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;
use simple_money::*;
use rust_decimal_macros::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TaxError {
    #[error("Mismatched currencies")]
    MismatchedCurrencies,
    #[error("Could not find deduction")]
    CouldNotFindDeduction,
    #[error("Could not find credit")]
    CouldNotFindCredit,
    #[error("Claim did not match strategy")]
    ClaimDidNotMatchStrategy
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TaxBracket{
    min_money: Money,
    max_money: Option<Money>,
    rate: Decimal,
}

impl PartialOrd for TaxBracket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.min_money.partial_cmp(&other.min_money)
    }
}

impl Ord for TaxBracket {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_money.cmp(&other.min_money)
    }
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

#[derive(Clone, Copy, Debug)]
pub enum ClaimStrategy {
    ExactAmount(Money),
    Range(Money, Money),
    Min(Money),
    Max(Money),
}

impl ClaimStrategy {
    fn is_claim_amount_valid(&self, claim_amount: Money) -> bool {
        match *self {
            ClaimStrategy::ExactAmount(exact_amount) => claim_amount == exact_amount,
            ClaimStrategy::Range(min_amount, max_amount) => claim_amount >= min_amount && claim_amount <= max_amount,
            ClaimStrategy::Min(min_amount) => claim_amount >= min_amount,
            ClaimStrategy::Max(max_amount) => claim_amount <= max_amount,
        }
    }

    pub fn apply_claim(&self, claim_amount: Money) -> Result<Money, TaxError> {
        if self.is_claim_amount_valid(claim_amount) {
           Ok(claim_amount)
        } else {
            Err(TaxError::ClaimDidNotMatchStrategy)
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaxCreditRule{
    pub refundable: bool,
    pub tax_credit_identifier: String,
    pub claim_strategy: ClaimStrategy,
}

impl TaxCreditRule {
    pub fn apply_credit(&self, credit_claim: &TaxCreditClaim) -> Result<Money, TaxError> {
       if self.tax_credit_identifier != credit_claim.tax_credit_identifier {
           return Err(TaxError::CouldNotFindCredit)
       } 

       self.claim_strategy.apply_claim(credit_claim.money_to_credit)
    }
}

#[derive(Clone)]
pub struct TaxCreditClaim{
    pub tax_credit_identifier: String,
    pub money_to_credit: Money,
}

#[derive(Debug, Clone)]
pub struct TaxDeductionRule {
    pub tax_deduction_identifier: String,
    pub claim_strategy: ClaimStrategy,
}

impl TaxDeductionRule {
    pub fn apply_deduction(&self, deduction_claim: &TaxDeductionClaim) -> Result<Money, TaxError> {
        if self.tax_deduction_identifier != deduction_claim.tax_deduction_identifier {
            return Err(TaxError::CouldNotFindDeduction)
        }

        self.claim_strategy.apply_claim(deduction_claim.money_to_deduct)
    }
}

#[derive(Clone)]
pub struct TaxDeductionClaim {
    pub tax_deduction_identifier: String,
    pub money_to_deduct: Money,
}

#[derive(Debug, Clone)]
pub struct TaxSchedule {
    brackets: Vec<TaxBracket>,
    deductions_map: HashMap<String, TaxDeductionRule>,
    credits_map: HashMap<String, TaxCreditRule>,
    tax_currency: Currency,
    capital_gains_inclusion_rate: Decimal,
}

#[derive(Clone,Copy)]
pub enum Income {
    Employment(Money),
    CapitalGains(Money),
}

#[derive(Clone,Copy)]
pub enum TaxCalculation {
    Refund(Money),
    Liability(Money),
}

impl TaxSchedule {
    fn validate_currency_on_bracket(bracket: &TaxBracket, currency: Currency) -> bool {
       if let Some(max_money) = bracket.max_money{
           max_money.currency == currency && bracket.min_money.currency == currency
       }else{
           bracket.min_money.currency == currency
       }
    }

    fn validate_currency_on_brackets(brackets: Vec<TaxBracket>, currency: Currency) -> bool {
        brackets.iter().all(|bracket| Self::validate_currency_on_bracket(bracket, currency))
    }

    fn determine_income_under_consideration_for_single_income_stream(&self, income: Income) -> Money {
        match income {
            Income::CapitalGains(capital_gains_income) => capital_gains_income * self.capital_gains_inclusion_rate,
            Income::Employment(employment_income) => employment_income,
        }
    }

    fn determine_income_to_consider(&self, incomes: Vec<Income>) -> Money {
       incomes.iter().map(|income| self.determine_income_under_consideration_for_single_income_stream(*income) ).fold(init_zero_amount(self.tax_currency), |acc, money| acc + money)
    }

    fn determine_taxable_income(&self, income_amount_under_consideration: Money, tax_deduction_claims: Vec<TaxDeductionClaim>) -> Money {
       let amount_to_deduct = tax_deduction_claims.iter().fold(init_zero_amount(self.tax_currency),|acc,tax_deduction_claim| {
           let tax_deduction_identifier = &tax_deduction_claim.tax_deduction_identifier;
           if let Some(tax_deduction) = self.deductions_map.get(tax_deduction_identifier){
               let deduction_amount = tax_deduction.apply_deduction(tax_deduction_claim).map_or_else(|_| init_zero_amount(self.tax_currency), |v| v);
               acc + deduction_amount
           }else{
               acc
           }
       });
       income_amount_under_consideration - amount_to_deduct
    }

    fn determine_tax_liability(&self, taxable_income: Money, tax_credit_claims: Vec<TaxCreditClaim>) -> Money {
        let (refundable_tax_credits, non_refundable_tax_credits): (Vec<(TaxCreditRule, TaxCreditClaim)>, Vec<(TaxCreditRule, TaxCreditClaim)>) = tax_credit_claims.iter().filter_map(|tax_credit_claim|{
            let tax_credit_identifier = &tax_credit_claim.tax_credit_identifier;
            if let Some(tax_credit_rule) = self.credits_map.get(tax_credit_identifier) {
                Some((tax_credit_rule.clone(), tax_credit_claim.clone()))
            }else{
                None
            }
        }).into_iter().partition(|(tax_credit_rule, _)| tax_credit_rule.refundable);

        let non_refundable_tax_credit_amount = non_refundable_tax_credits.iter().fold(init_zero_amount(self.tax_currency), |acc, (tax_credit_rule, tax_credit_claim)| {
            let tax_credit_amount = tax_credit_rule.apply_credit(tax_credit_claim).map_or_else(|_| init_zero_amount(self.tax_currency), |v| v);
            acc + tax_credit_amount 
        });
        let refundable_tax_credit_amount = refundable_tax_credits.iter().fold(init_zero_amount(self.tax_currency), |acc, (tax_credit_rule, tax_credit_claim)| {
            let tax_credit_amount = tax_credit_rule.apply_credit(tax_credit_claim).map_or_else(|_| init_zero_amount(self.tax_currency), |v| v);
            acc + tax_credit_amount
        });

        let taxable_income_less_non_refundable_tax_credits = taxable_income - non_refundable_tax_credit_amount;

        if taxable_income_less_non_refundable_tax_credits.amount < dec!(0) {
            init_zero_amount(self.tax_currency) - refundable_tax_credit_amount
        }else{
            taxable_income_less_non_refundable_tax_credits - refundable_tax_credit_amount
        }
    }

    pub fn calculate_tax_result(&self, incomes: Vec<Income>, tax_deduction_claims: Vec<TaxDeductionClaim>, tax_credit_claims: Vec<TaxCreditClaim>) -> TaxCalculation {
        let income_to_consider = self.determine_income_to_consider(incomes);
        let taxable_income = self.determine_taxable_income(income_to_consider, tax_deduction_claims);
        let tax_liability = self.determine_tax_liability(taxable_income, tax_credit_claims);

        if tax_liability.amount < dec!(0) {
            let tax_refund = Money { amount: tax_liability.amount * dec!(-1), currency: tax_liability.currency };
            TaxCalculation::Refund(tax_refund)
        } else{
            TaxCalculation::Liability(tax_liability)
        }
    }

    pub fn new(
        brackets: Vec<TaxBracket>,
        currency: Currency,
        capital_gains_inclusion_rate: Decimal,
    ) -> Result<TaxSchedule, TaxError> {
        if !Self::validate_currency_on_brackets(brackets.clone(), currency){
           Err(TaxError::MismatchedCurrencies) 
        }else{
            let mut new_brackets = brackets.clone();
            new_brackets.sort();
            return Ok(TaxSchedule {
                brackets: new_brackets,
                deductions_map: HashMap::new(),
                credits_map: HashMap::new(),
                tax_currency: currency,
                capital_gains_inclusion_rate: capital_gains_inclusion_rate,
            })
        }
    }

    pub fn add_deduction(
        &mut self,
        tax_deduction_identifier: String,
        tax_deduction_rule: TaxDeductionRule,
    ){
        self.deductions_map.insert(tax_deduction_identifier, tax_deduction_rule);
    }

    pub fn add_credit(
        &mut self,
        tax_credit_identifier: String,
        tax_credit_rule: TaxCreditRule,
    ){
        self.credits_map.insert(tax_credit_identifier, tax_credit_rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let schedule = TaxSchedule::new(vec![lowest, middle, highest], Currency::CAD).unwrap();

        let over_highest_tax = schedule.calculate_tax(cad_money!(25_000));
        assert_eq!(over_highest_tax, cad_money!(6_500));

        let middle_tax = schedule.calculate_tax(cad_money!(15_000));
        assert_eq!(middle_tax, cad_money!(2000));

        let lowest_tax = schedule.calculate_tax(cad_money!(5_000));
        assert_eq!(lowest_tax, cad_money!(500));
    }

    #[test]
    fn single_bracket_example() {
        let lowest = TaxBracket {
            min_money: cad_money!(0),
            max_money: Some(cad_money!(10_000)),
            rate: dec!(0.1),
        };

        let schedule = TaxSchedule::new(vec![lowest], Currency::CAD).unwrap();
        let tax = schedule.calculate_tax(cad_money!(10_000));

        assert_eq!(tax, cad_money!(1000));
    }

    #[test]
    fn invalid_bracket_and_regime(){
        let invalid = TaxBracket::new(
            cad_money!(0), 
            Some(usd_money!(1)), 
            dec!(0.1)
        ).unwrap_err();

        assert_eq!(invalid, TaxError::MismatchedCurrencies);

        let valid_bracket = TaxBracket::new(
            cad_money!(0),
            None,
            dec!(0.1)
        ).unwrap();
        let invalid_schedule = TaxSchedule::new(
            vec![valid_bracket],
            Currency::USD,
        ).unwrap_err();

        assert_eq!(invalid_schedule, TaxError::MismatchedCurrencies);
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

        let mut schedule = TaxSchedule::new(
            vec![single],
            Currency::CAD,
        ).unwrap();
        schedule.set_deduction(
            TaxDeductionCategory::CapitalGains, 
            capital_gains_deduction
        );
        let actual_deductions = vec![TaxDeduction {
            tax_deduction_type: TaxDeductionCategory::CapitalGains,
            money_to_deduct: cad_money!(5000),
        }];
        let tax = schedule.calculate_tax_with_deductions(cad_money!(10_000), actual_deductions);

        match tax {
            Ok(result) => assert_eq!(result, cad_money!(750.00)),
            Err(_) => assert!(false, "Tax should not be an Err"),
        }
    }
}