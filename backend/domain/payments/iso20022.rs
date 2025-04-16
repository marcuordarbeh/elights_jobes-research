// domain/payments/iso20022.rs

/// Dummy implementation for building an ISO 20022 payment message.
/// In production, you would use detailed ISO 20022 libraries and schema.
pub fn build_iso20022_message(payment_type: &str, amount: f64, account: &str) -> String {
    format!(
        "<ISO20022Message>
            <PaymentType>{}</PaymentType>
            <Amount>{}</Amount>
            <Account>{}</Account>
         </ISO20022Message>",
        payment_type, amount, account
    )
}
