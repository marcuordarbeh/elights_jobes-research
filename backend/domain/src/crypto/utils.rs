// /home/inno/elights_jobes-research/backend/domain/src/crypto/utils.rs
use rust_decimal::Decimal;
use rust_decimal_macros::dec; // For decimal literals

// Constants for Satoshis and Monero atomic units
const SATOSHIS_PER_BTC: Decimal = dec!(100_000_000);
const ATOMIC_UNITS_PER_XMR: Decimal = dec!(1_000_000_000_000);

/// Converts a BTC amount (as Decimal) to Satoshis (as u64).
/// Returns None if conversion results in overflow or is negative.
pub fn btc_to_satoshis(btc_amount: Decimal) -> Option<u64> {
    if btc_amount < Decimal::ZERO {
        return None;
    }
    (btc_amount * SATOSHIS_PER_BTC).to_u64()
}

/// Converts Satoshis (as u64) to BTC amount (as Decimal).
pub fn satoshis_to_btc(satoshis: u64) -> Decimal {
    Decimal::from(satoshis) / SATOSHIS_PER_BTC
}

/// Converts an XMR amount (as Decimal) to Monero atomic units (piconeros) (as u64).
/// Returns None if conversion results in overflow or is negative.
pub fn xmr_to_atomic_units(xmr_amount: Decimal) -> Option<u64> {
    if xmr_amount < Decimal::ZERO {
        return None;
    }
    (xmr_amount * ATOMIC_UNITS_PER_XMR).to_u64()
}

/// Converts Monero atomic units (as u64) to XMR amount (as Decimal).
pub fn atomic_units_to_xmr(atomic_units: u64) -> Decimal {
    Decimal::from(atomic_units) / ATOMIC_UNITS_PER_XMR
}

// Tests remain the same as previously generated
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_btc_satoshi_conversion() {
        assert_eq!(btc_to_satoshis(dec!(1.2345)), Some(123450000));
        assert_eq!(satoshis_to_btc(123450000), dec!(1.2345));
        assert_eq!(btc_to_satoshis(dec!(-1.0)), None);
    }

    #[test]
    fn test_xmr_atomic_conversion() {
        assert_eq!(xmr_to_atomic_units(dec!(0.5)), Some(500_000_000_000));
        assert_eq!(atomic_units_to_xmr(500_000_000_000), dec!(0.5));
         assert_eq!(xmr_to_atomic_units(dec!(-1.0)), None);
    }
}