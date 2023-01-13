use std::ops::Mul;

fn main() {}

#[derive(Debug, PartialEq)]
struct Money {
    currency: Currency,
    amount: f64,
}

#[derive(Clone, PartialEq, Debug)]
enum Currency {
    USD,
    EUR,
    KRW,
}

// impl<T> Money where T: std::ops::Mul<T> + Clone + std::convert::Into<f64>,
impl Money {
    /// Multiplication of Money by a certain number.
    fn times(&self, multiplier: f64) -> Money {
        Self {
            currency: self.currency.clone(),
            amount: self.amount * multiplier,
        }
    }
    /// Division of Money by a certain number.
    fn divide(&self, divisor: f64) -> Money {
        let result = if divisor != 0.0 {
            self.amount / divisor
        } else {
            0.0
        };
        Self {
            currency: self.currency.clone(),
            amount: result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Currency, Money};

    // Chapter 1
    #[test]
    fn check_basic_amount_method() {
        // Arange
        let fiver = Money {
            currency: Currency::USD,
            amount: 5.0,
        };
        // Act
        let tenner = fiver.times(2.0);
        // Assert
        assert_eq!(tenner.amount, 5.0 * 2.0);
    }

    // Chapter 2
    #[test]
    fn check_basic_currency_creation() {
        // Arange
        let ten_euros = Money {
            currency: Currency::EUR,
            amount: 10.0,
        };
        // Act
        let twenty_euros = ten_euros.times(2.0);
        // Assert
        assert_eq!(twenty_euros.amount, 20.0);
        assert_eq!(twenty_euros.currency, Currency::EUR);
    }

    #[test]
    fn check_basic_currency_division() {
        let original_money = Money {
            currency: Currency::KRW,
            amount: 4002.0,
        };
        let actual_money_after_division = original_money.divide(4.0);
        let expected_money_after_division = Money {
            currency: Currency::KRW,
            amount: 1000.5,
        };
        assert_eq!(actual_money_after_division, expected_money_after_division);
    }
}
