
#[derive(Clone)]
pub struct TaxBracket {
    min: f64,
    max: f64,
    rate: f64,
}

#[derive(Clone)]
pub struct TaxRegime {
    brackets: Vec<TaxBracket>,
}

impl TaxRegime {
    pub fn new(brackets: Vec<TaxBracket>) -> TaxRegime {
        let mut new_brackets = brackets.clone();
        new_brackets.sort_by(|a,b| a.min.partial_cmp(&b.min).unwrap());
        return TaxRegime { brackets: new_brackets }
    }
}

fn calculate_tax_for_bracket(taxable_income: f64, tax_bracket: &TaxBracket) -> f64 {
    if taxable_income < tax_bracket.min {
        return 0.0
    }

    if taxable_income > tax_bracket.max {
        return tax_bracket.max * tax_bracket.rate;
    }

    return (taxable_income - tax_bracket.min) * tax_bracket.rate;
}

pub fn calculate_tax(taxable_income: f64, tax_regime: TaxRegime) -> f64 {
    tax_regime.brackets.iter().map(|bracket| calculate_tax_for_bracket(taxable_income, bracket)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let lowest = TaxBracket { min: 0.0, max: 10000.0, rate: 0.1};
        let middle = TaxBracket { min: 10000.0, max: 20000.0, rate: 0.2};
        let highest = TaxBracket { min: 20000.0, max: f64::INFINITY, rate: 0.3};

        let regime = TaxRegime::new(vec![lowest,middle,highest]);

        let over_highest_tax = calculate_tax(25000.0, regime.clone());
        assert_eq!(over_highest_tax, 6500.0);

        let middle_tax = calculate_tax(15000.0, regime.clone());
        assert_eq!(middle_tax, 2000.0);

        let lowest_tax = calculate_tax(5000.0, regime);
        assert_eq!(lowest_tax, 500.0);
    }

    #[test]
    fn single_bracket_example(){
        let lowest = TaxBracket { min: 0.0, max: 10000.0, rate: 0.1};

        let regime = TaxRegime::new(vec![lowest]);
        let tax = calculate_tax(10000.0, regime);

        assert_eq!(tax, 1000.0);
    }
}
