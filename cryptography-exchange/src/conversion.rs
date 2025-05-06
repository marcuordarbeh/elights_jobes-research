// /home/inno/elights_jobes-research/cryptography-exchange/src/conversion.rs
use crate::error::ExchangeError;
use crate::models::{ConversionQuote, ConversionRequest, ConversionResult};
use crate::rate_service::RateService; // Use the RateService trait
use rust_decimal::Decimal;
use async_trait::async_trait;

/// Trait for performing currency conversions.
#[async_trait]
pub trait CurrencyConverter: Send + Sync {
    /// Gets a quote for converting between two currencies.
    async fn get_quote(
        &self,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<ConversionQuote, ExchangeError>;

    /// Executes a currency conversion.
    async fn execute_conversion(
        &self,
        request: &ConversionRequest,
    ) -> Result<ConversionResult, ExchangeError>;
}

// --- Implementation using a Rate Service ---

/// Simple converter that uses fetched rates. Does not execute trades.
pub struct SimpleRateConverter {
    rate_service: Box<dyn RateService>, // Inject RateService implementation
}

impl SimpleRateConverter {
    pub fn new(rate_service: Box<dyn RateService>) -> Self {
        Self { rate_service }
    }
}

#[async_trait]
impl CurrencyConverter for SimpleRateConverter {
    async fn get_quote(
        &self,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<ConversionQuote, ExchangeError> {
        log::debug!("Getting conversion quote: {} -> {}", from_currency, to_currency);
        self.rate_service.get_rate(from_currency, to_currency).await
        // Result of get_rate should match ConversionQuote structure or be adapted
    }

    async fn execute_conversion(
        &self,
        request: &ConversionRequest,
    ) -> Result<ConversionResult, ExchangeError> {
        log::info!("Executing simple conversion: {} {} -> {}",
            request.amount, request.from_currency, request.to_currency);

        if request.amount <= Decimal::ZERO {
            return Err(ExchangeError::InvalidInput("Conversion amount must be positive".to_string()));
        }

        // 1. Get the current rate
        let quote = self.get_quote(&request.from_currency, &request.to_currency).await?;

        // 2. Perform calculation
        // TODO: Add spread/fees based on configuration if this converter handles it
        let converted_amount = request.amount * quote.rate;

        // Note: This implementation doesn't actually *execute* a trade on an exchange.
        // It just calculates the result based on the fetched rate.
        // A real exchange integration would involve placing orders via API.

        Ok(ConversionResult {
            from_currency: request.from_currency.clone(),
            to_currency: request.to_currency.clone(),
            original_amount: request.amount,
            converted_amount,
            rate_used: quote.rate,
            exchange_reference_id: None, // No external execution ID for simple conversion
        })
    }
}


// --- Mock Implementation for Testing ---
#[derive(Default, Clone)]
pub struct MockCurrencyConverter {
    pub mock_rate: Option<Decimal>,
    pub should_fail: bool,
}

#[async_trait]
impl CurrencyConverter for MockCurrencyConverter {
    async fn get_quote(
        &self,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<ConversionQuote, ExchangeError> {
        if self.should_fail {
            return Err(ExchangeError::InternalError("Mock Quote Failure".to_string()));
        }
        let rate = self.mock_rate.unwrap_or_else(|| {
            // Provide some default mock rate if not set
            match (from_currency, to_currency) {
                ("BTC", "USD") => Decimal::new(50000, 0),
                ("USD", "BTC") => Decimal::new(2, 8), // 1 / 50000
                ("XMR", "USD") => Decimal::new(150, 0),
                ("USD", "XMR") => Decimal::new(666666, 8), // ~ 1 / 150
                _ => Decimal::ONE, // Default 1:1 rate for other pairs
            }
        });
        Ok(ConversionQuote {
            from_currency: from_currency.to_string(),
            to_currency: to_currency.to_string(),
            rate,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn execute_conversion(
        &self,
        request: &ConversionRequest,
    ) -> Result<ConversionResult, ExchangeError> {
         if self.should_fail {
            return Err(ExchangeError::InternalError("Mock Execution Failure".to_string()));
        }
         let quote = self.get_quote(&request.from_currency, &request.to_currency).await?;
         let converted_amount = request.amount * quote.rate;
         Ok(ConversionResult {
            from_currency: request.from_currency.clone(),
            to_currency: request.to_currency.clone(),
            original_amount: request.amount,
            converted_amount,
            rate_used: quote.rate,
            exchange_reference_id: Some(format!("MOCK_CONV_{}", rand::random::<u32>())),
        })
    }
}