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

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
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

    fn determine_taxable_income(&self, income_amount_under_consideration: Money, tax_deduction_claims: Vec<TaxDeductionClaim>) -> Result<Money, TaxError> {
        let amount_to_deduct = tax_deduction_claims.iter().try_fold(init_zero_amount(self.tax_currency), |acc, tax_deduction_claim| {
           let tax_deduction_identifier = tax_deduction_claim.tax_deduction_identifier.clone();
           if let Some(tax_deduction) = self.deductions_map.get(&tax_deduction_identifier){
               let deduction_amount = tax_deduction.apply_deduction(&tax_deduction_claim)?;
               Ok(acc + deduction_amount)
           }else{
               Ok(acc)
           }
        })?;

        let taxable_income = income_amount_under_consideration - amount_to_deduct;
       
       if taxable_income.amount < dec!(0) {
           Ok(init_zero_amount(self.tax_currency))
       }else{
           Ok(taxable_income)
       }
    }

    fn determine_tax_liability(&self, taxable_income: Money) -> Money {
        self.brackets.iter().map(|bracket| bracket.calculate_tax(taxable_income)).fold(init_zero_amount(self.tax_currency), |acc, tax_liability| acc + tax_liability)
    }

    fn determine_tax_liability_or_refund(&self, tax_liability: Money, tax_credit_claims: Vec<TaxCreditClaim>) -> Result<TaxCalculation, TaxError> {
        let (refundable_tax_credits, non_refundable_tax_credits): (Vec<(TaxCreditRule, TaxCreditClaim)>, Vec<(TaxCreditRule, TaxCreditClaim)>) = tax_credit_claims.iter().filter_map(|tax_credit_claim|{
            let tax_credit_identifier = &tax_credit_claim.tax_credit_identifier;
            if let Some(tax_credit_rule) = self.credits_map.get(tax_credit_identifier) {
                Some((tax_credit_rule.clone(), tax_credit_claim.clone()))
            }else{
                None
            }
        }).into_iter().partition(|(tax_credit_rule, _)| tax_credit_rule.refundable);

        let non_refundable_tax_credit_amount = non_refundable_tax_credits.iter().try_fold(init_zero_amount(self.tax_currency), |acc, (tax_credit_rule, tax_credit_claim)| {
            let tax_credit_amount = tax_credit_rule.apply_credit(tax_credit_claim)?;
            Ok(acc + tax_credit_amount)
        })?;
        let refundable_tax_credit_amount = refundable_tax_credits.iter().try_fold(init_zero_amount(self.tax_currency), |acc, (tax_credit_rule, tax_credit_claim)| {
            let tax_credit_amount = tax_credit_rule.apply_credit(tax_credit_claim)?;
            Ok(acc + tax_credit_amount)
        })?;

        let tax_liability_less_non_refundable_tax_credits = tax_liability - non_refundable_tax_credit_amount;

        if tax_liability_less_non_refundable_tax_credits.amount < dec!(0) {
            Ok(TaxCalculation::Refund(refundable_tax_credit_amount))
        }else{
            let difference = tax_liability_less_non_refundable_tax_credits.amount - refundable_tax_credit_amount.amount;
            let abs_diff = difference.abs();
            let is_liability = difference.is_sign_positive();
            let money = Money{ amount: abs_diff, currency: self.tax_currency };

            return if is_liability {
                Ok(TaxCalculation::Liability(money))
            } else {
                Ok(TaxCalculation::Refund(money))
            }
        }
    }

    pub fn calculate_tax_result(&self, incomes: Vec<Income>, tax_deduction_claims: Vec<TaxDeductionClaim>, tax_credit_claims: Vec<TaxCreditClaim>) -> Result<TaxCalculation, TaxError> {
        let income_to_consider = self.determine_income_to_consider(incomes);
        let taxable_income = self.determine_taxable_income(income_to_consider, tax_deduction_claims)?;
        let tax_liability = self.determine_tax_liability(taxable_income);
        self.determine_tax_liability_or_refund(tax_liability, tax_credit_claims)
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
        tax_deduction_rule: TaxDeductionRule,
    ){
        self.deductions_map.insert(
            tax_deduction_rule.tax_deduction_identifier.clone(), 
            tax_deduction_rule
        );
    }

    pub fn add_credit(
        &mut self,
        tax_credit_rule: TaxCreditRule,
    ){
        self.credits_map.insert(
            tax_credit_rule.tax_credit_identifier.clone(), 
            tax_credit_rule
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_tax_without_deductions_and_credits_with_three_brackets() {
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

        let schedule = TaxSchedule::new(vec![lowest, middle, highest], Currency::CAD, dec!(0.5)).unwrap();

        let twenty_five_thousand_employment_income = Income::Employment(cad_money!(25_000));
        let fifteen_thousand_employment_income = Income::Employment(cad_money!(15_000));
        let five_thousand_employment_income = Income::Employment(cad_money!(5_000));
        let five_thousand_capital_gains = Income::CapitalGains(cad_money!(5_000));

        let over_highest_tax = schedule.calculate_tax_result(vec![twenty_five_thousand_employment_income], vec![], vec![]).unwrap();
        assert_eq!(over_highest_tax, TaxCalculation::Liability(cad_money!(6_500)));

        let over_highest_tax_with_capital_gains = schedule.calculate_tax_result(vec![twenty_five_thousand_employment_income, five_thousand_capital_gains], vec![], vec![]).unwrap();
        assert_eq!(over_highest_tax_with_capital_gains, TaxCalculation::Liability(cad_money!(8_000)));

        let middle_tax = schedule.calculate_tax_result(vec![fifteen_thousand_employment_income], vec![], vec![]).unwrap();
        assert_eq!(middle_tax, TaxCalculation::Liability(cad_money!(2_000)));

        let lowest_tax = schedule.calculate_tax_result(vec![five_thousand_employment_income], vec![], vec![]).unwrap();
        assert_eq!(lowest_tax, TaxCalculation::Liability(cad_money!(500)));
    }

    #[test]
    fn calculate_tax_without_deductions_and_credits_with_single_bracket() {
        let lowest = TaxBracket {
            min_money: cad_money!(0),
            max_money: Some(cad_money!(10_000)),
            rate: dec!(0.1),
        };

        let schedule = TaxSchedule::new(vec![lowest], Currency::CAD, dec!(0.5)).unwrap();
        let employment_income = Income::Employment(cad_money!(10_000));
        let capital_gains = Income::CapitalGains(cad_money!(10_000));

        let tax_on_employment_income = schedule.calculate_tax_result(vec![employment_income], vec![], vec![]).unwrap();
        let tax_on_capital_gains_and_employment_income = schedule.calculate_tax_result(vec![employment_income, capital_gains], vec![], vec![]).unwrap();

        assert_eq!(tax_on_employment_income, TaxCalculation::Liability(cad_money!(1000)));
        assert_eq!(tax_on_capital_gains_and_employment_income, TaxCalculation::Liability(cad_money!(2000)));
    }

    #[test]
    fn bracket_and_schedule_with_currencies_that_dont_match_fails(){
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
            dec!(0.5),
        ).unwrap_err();

        assert_eq!(invalid_schedule, TaxError::MismatchedCurrencies);
    }

    #[test]
    fn many_bracket_with_deductions_and_credits() {
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

        let rrsp_deduction_max = TaxDeductionRule {
            tax_deduction_identifier: String::from("RRSP_MAX"),
            claim_strategy: ClaimStrategy::Max(cad_money!(5_000)),
        };
        let rrsp_deduction_min = TaxDeductionRule {
            tax_deduction_identifier: String::from("RRSP_MIN"),
            claim_strategy: ClaimStrategy::Min(cad_money!(5_000)),
        };
        let rrsp_deduction_exact = TaxDeductionRule {
            tax_deduction_identifier: String::from("RRSP_EXACT"),
            claim_strategy: ClaimStrategy::ExactAmount(cad_money!(5_000)),
        };
        let rrsp_deduction_range = TaxDeductionRule {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            claim_strategy: ClaimStrategy::Range(cad_money!(2_500), cad_money!(5_000)),
        };

        let mut schedule = TaxSchedule::new(
            vec![lowest, middle, highest],
            Currency::CAD,
            dec!(0.5),
        ).unwrap();

        schedule.add_deduction(rrsp_deduction_max);
        schedule.add_deduction(rrsp_deduction_min);
        schedule.add_deduction(rrsp_deduction_exact);
        schedule.add_deduction(rrsp_deduction_range);

        let valid_max_deduction_claim_at_bound = TaxDeductionClaim { 
            tax_deduction_identifier: String::from("RRSP_MAX"),
            money_to_deduct: cad_money!(5_000),
        };
        let valid_max_deduction_claim_within = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_MAX"),
            money_to_deduct: cad_money!(2_500),
        }; 
        let invalid_max_deduction = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_MAX"),
            money_to_deduct: cad_money!(6_000),
        };

        let valid_min_deduction_claim_at_bound = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_MIN"),
            money_to_deduct: cad_money!(5_000),
        };
        let valid_min_deduction_claim_within = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_MIN"),
            money_to_deduct: cad_money!(6_000),
        };
        let invalid_min_deduction_claim = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_MIN"),
            money_to_deduct: cad_money!(2_500),
        };

        let valid_exact_deduction_claim = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_EXACT"),
            money_to_deduct: cad_money!(5_000),
        };
        let invalid_exact_deduction_claim = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_EXACT"),
            money_to_deduct: cad_money!(5_000),
        };

        let valid_range_deduction_claim_at_min_bound = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            money_to_deduct: cad_money!(2_500),
        };
        let valid_range_deduction_claim_at_max_bound = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            money_to_deduct: cad_money!(5_000),
        };
        let valid_range_deduction_claim_within = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            money_to_deduct: cad_money!(3_000),
        };
        let invalid_range_deduction_claim_past_max = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            money_to_deduct: cad_money!(6_000),
        };
        let invalid_range_deduction_claim_past_min = TaxDeductionClaim {
            tax_deduction_identifier: String::from("RRSP_RANGE"),
            money_to_deduct: cad_money!(1_000),
        };

        let employment_income = Income::Employment(cad_money!(25_000));

        let max_at_bound_claim_result = schedule.calculate_tax_result(
            vec![employment_income], 
            vec![valid_max_deduction_claim_at_bound], 
            vec![]
        ).unwrap(); 
        let max_within_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_max_deduction_claim_within],
            vec![],
        ).unwrap();
        let invalid_max_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![invalid_max_deduction],
            vec![],
        ).unwrap_err();

        assert_eq!(max_at_bound_claim_result, TaxCalculation::Liability(cad_money!(3_000)));
        assert_eq!(max_within_claim_result, TaxCalculation::Liability(cad_money!(3_750)));
        assert_eq!(invalid_max_claim_result, TaxError::ClaimDidNotMatchStrategy);

        let min_at_bound_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_min_deduction_claim_at_bound],
            vec![],
        ).unwrap();
        let min_within_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_min_deduction_claim_within],
            vec![],
        ).unwrap();
        let invalid_min_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![invalid_min_deduction_claim],
            vec![],
        ).unwrap_err();

        assert_eq!(min_at_bound_claim_result, TaxCalculation::Liability(cad_money!(3_000)));
        assert_eq!(min_within_claim_result, TaxCalculation::Liability(cad_money!(2_800)));
        assert_eq!(invalid_min_claim_result, TaxError::ClaimDidNotMatchStrategy);

        let exact_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_exact_deduction_claim.clone()],
            vec![],
        ).unwrap();
        let invalid_exact_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![invalid_exact_deduction_claim.clone()],
            vec![],
        ).unwrap_err();

        assert_eq!(exact_claim_result, TaxCalculation::Liability(cad_money!(3_000)));
        assert_eq!(invalid_exact_claim_result, TaxError::ClaimDidNotMatchStrategy);

        let range_deduction_claim_at_max_bound_result = schedule.calculate_tax_result(
            vec![employment_income], 
            vec![valid_range_deduction_claim_at_max_bound], 
            vec![],
        ).unwrap();
        let range_deduction_claim_within_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_range_deduction_claim_within],
            vec![],
        ).unwrap();
        let range_deduction_claim_at_min_bound_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_range_deduction_claim_at_min_bound],
            vec![],
        ).unwrap();
        let invalid_range_deduction_claim_past_min = schedule.calculate_tax_result(
            vec![employment_income],
            vec![invalid_range_deduction_claim_past_min],
            vec![],
        ).unwrap_err();
        let invalid_range_deduction_claim_past_max = schedule.calculate_tax_result(
            vec![employment_income],
            vec![invalid_range_deduction_claim_past_max],
            vec![],
        ).unwrap_err();

        assert_eq!(range_deduction_claim_at_max_bound_result, TaxCalculation::Liability(cad_money!(3_000)));
        assert_eq!(range_deduction_claim_within_result, TaxCalculation::Liability(cad_money!(3_600)));
        assert_eq!(range_deduction_claim_at_min_bound_result, TaxCalculation::Liability(cad_money!(3_750)));
        assert_eq!(invalid_range_deduction_claim_past_min, TaxError::ClaimDidNotMatchStrategy);
        assert_eq!(invalid_range_deduction_claim_past_max, TaxError::ClaimDidNotMatchStrategy);

        let non_refundable_full_credit = TaxCreditRule {
            tax_credit_identifier: String::from("NON_REFUNDABLE_FULL_CREDIT"),
            claim_strategy: ClaimStrategy::ExactAmount(cad_money!(25_000)),
            refundable: false,
        };
        let refundable_full_credit = TaxCreditRule {
            tax_credit_identifier: String::from("REFUNDABLE_FULL_CREDIT"),
            claim_strategy: ClaimStrategy::ExactAmount(cad_money!(25_000)),
            refundable: true,
        };

        schedule.add_credit(non_refundable_full_credit);
        schedule.add_credit(refundable_full_credit);

        let non_refundable_full_credit_claim = TaxCreditClaim {
            tax_credit_identifier: String::from("NON_REFUNDABLE_FULL_CREDIT"),
            money_to_credit: cad_money!(25_000),
        };
        let refundable_full_credit_claim = TaxCreditClaim {
            tax_credit_identifier: String::from("REFUNDABLE_FULL_CREDIT"),
            money_to_credit: cad_money!(25_000),
        };

        let non_refundable_full_credit_claim_result = schedule.calculate_tax_result(
            vec![employment_income],
            vec![valid_exact_deduction_claim.clone()],
            vec![non_refundable_full_credit_claim],
        ).unwrap();
        let refundable_full_credit_claim_result = schedule.calculate_tax_result(
            vec![employment_income], 
            vec![valid_exact_deduction_claim], 
            vec![refundable_full_credit_claim],
        ).unwrap();

        assert_eq!(non_refundable_full_credit_claim_result, TaxCalculation::Liability(cad_money!(0)));
        assert_eq!(refundable_full_credit_claim_result, TaxCalculation::Refund(cad_money!(5_000)));
    }
}