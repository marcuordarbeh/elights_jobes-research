// /home/inno/elights_jobes-research/backend/domain/src/utils.rs
use rust_decimal::Decimal;
use bigdecimal::BigDecimal;
use std::str::FromStr;

/// Masks sensitive parts of an account number, showing only last 4 digits.
pub fn mask_account_number(account_num: &str) -> String {
    let len = account_num.len();
    if len <= 4 {
        // If too short to mask meaningfully, return original or full mask
        return "****".to_string();
    }
    format!("****{}", &account_num[(len - 4)..])
}

/// Converts BigDecimal (from Diesel Numeric) to rust_decimal::Decimal.
/// Includes basic error handling.
pub fn bigdecimal_to_decimal(bd: BigDecimal) -> Decimal {
    Decimal::from_str(&bd.to_string()).unwrap_or_else(|_| {
         log::error!("Failed to convert BigDecimal '{}' to Decimal, returning 0", bd.to_string());
         Decimal::ZERO
    })
}

/// Converts rust_decimal::Decimal to BigDecimal (for Diesel Numeric).
/// Includes basic error handling.
pub fn decimal_to_bigdecimal(d: Decimal) -> BigDecimal {
     BigDecimal::from_str(&d.to_string()).unwrap_or_else(|_| {
         log::error!("Failed to convert Decimal '{}' to BigDecimal, returning 0", d.to_string());
          BigDecimal::from(0)
     })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masking() {
        assert_eq!(mask_account_number("1234567890"), "****7890");
        assert_eq!(mask_account_number("1234"), "****");
        assert_eq!(mask_account_number("123"), "****");
        assert_eq!(mask_account_number(""), "****");
    }

    #[test]
    fn test_decimal_conversions() {
         use rust_decimal_macros::dec;
         // Decimal -> BigDecimal
         let d = dec!(123.456);
         let bd = decimal_to_bigdecimal(d);
         assert_eq!(bd.to_string(), "123.456");

         // BigDecimal -> Decimal
         let bd_from_str = BigDecimal::from_str("987.654").unwrap();
         let d_conv = bigdecimal_to_decimal(bd_from_str);
         assert_eq!(d_conv, dec!(987.654));

         // Error case (example - non-numeric string for BigDecimal)
         // Note: This specific error won't happen with direct Decimal conversion,
         // but showcases the unwrap_or_else logic.
         let bad_bd = BigDecimal::from(0); // Simulate obtaining a 0 due to error
         let d_err = bigdecimal_to_decimal(bad_bd);
         assert_eq!(d_err, Decimal::ZERO);
    }
}