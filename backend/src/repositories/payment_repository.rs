use sqlx::PgPool;

pub async fn save_ach_details(details: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO ach_details (details) VALUES ($1)", details)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn save_bank_transfer_details(bank_name: &str, account_number: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO bank_transfers (bank_name, account_number) VALUES ($1, $2)", bank_name, account_number)
        .execute(pool)
        .await?;
    Ok(())
}