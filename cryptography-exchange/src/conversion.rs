// /home/inno/elights_jobes-research/cryptography-exchange/src/conversion.rs
use rust_decimal::Decimal;
use crate::error::ExchangeError;

/// Converts BTC to a fiat currency (e.g., USD, EUR) using a given rate.
pub fn btc_to_fiat(
    btc_amount: Decimal,
    btc_fiat_rate: Decimal,
) -> Result<Decimal, ExchangeError> {
    if btc_fiat_rate <= Decimal::ZERO {
        return Err(ExchangeError::ConversionError(
            "BTC to Fiat rate must be positive".to_string(),
        ));
    }
    Ok(btc_amount * btc_fiat_rate)
}

/// Converts a fiat currency (e.g., USD, EUR) to BTC using a given rate.
pub fn fiat_to_btc(
    fiat_amount: Decimal,
    btc_fiat_rate: Decimal,
) -> Result<Decimal, ExchangeError> {
    if btc_fiat_rate <= Decimal::ZERO {
        return Err(ExchangeError::ConversionError(
            "Fiat to BTC rate must be positive".to_string(),
        ));
    }
    fiat_amount
        .checked_div(btc_fiat_rate)
        .ok_or_else(|| ExchangeError::ConversionError("Division by zero or overflow".to_string()))
}

/// Converts XMR to a fiat currency (e.g., USD, EUR) using a given rate.
pub fn xmr_to_fiat(
    xmr_amount: Decimal,
    xmr_fiat_rate: Decimal,
) -> Result<Decimal, ExchangeError> {
    if xmr_fiat_rate <= Decimal::ZERO {
         return Err(ExchangeError::ConversionError(
            "XMR to Fiat rate must be positive".to_string(),
        ));
    }
    Ok(xmr_amount * xmr_fiat_rate)
}

/// Converts a fiat currency (e.g., USD, EUR) to XMR using a given rate.
pub fn fiat_to_xmr(
    fiat_amount: Decimal,
    xmr_fiat_rate: Decimal,
) -> Result<Decimal, ExchangeError> {
     if xmr_fiat_rate <= Decimal::ZERO {
        return Err(ExchangeError::ConversionError(
            "Fiat to XMR rate must be positive".to_string(),
        ));
    }
     fiat_amount
        .checked_div(xmr_fiat_rate)
        .ok_or_else(|| ExchangeError::ConversionError("Division by zero or overflow".to_string()))
}

// TODO: Implement functions to fetch real-time conversion rates from an external API.
// pub async fn get_btc_usd_rate() -> Result<Decimal, ExchangeError> { ... }
// pub async fn get_xmr_usd_rate() -> Result<Decimal, ExchangeError> { ... }

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_fiat_btc_conversions() {
        let rate = dec!(50000.0);
        assert_eq!(btc_to_fiat(dec!(1.5), rate).unwrap(), dec!(75000.0));
        assert_eq!(fiat_to_btc(dec!(75000.0), rate).unwrap(), dec!(1.5));
        assert!(fiat_to_btc(dec!(100.0), dec!(0.0)).is_err());
    }

     #[test]
    fn test_fiat_xmr_conversions() {
        let rate = dec!(200.0);
        assert_eq!(xmr_to_fiat(dec!(2.5), rate).unwrap(), dec!(500.0));
        assert_eq!(fiat_to_xmr(dec!(500.0), rate).unwrap(), dec!(2.5));
        assert!(xmr_to_fiat(dec!(1.0), dec!(-1.0)).is_err());
    }
}