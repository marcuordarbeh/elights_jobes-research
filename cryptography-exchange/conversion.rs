// cryptography-exchange/conversion.rs

pub fn btc_to_usd(btc_amount: f64, btc_usd_rate: f64) -> f64 {
    btc_amount * btc_usd_rate
}

pub fn usd_to_btc(usd_amount: f64, btc_usd_rate: f64) -> f64 {
    usd_amount / btc_usd_rate
}

pub fn xmr_to_usd(xmr_amount: f64, xmr_usd_rate: f64) -> f64 {
    xmr_amount * xmr_usd_rate
}

pub fn usd_to_xmr(usd_amount: f64, xmr_usd_rate: f64) -> f64 {
    usd_amount / xmr_usd_rate
}
