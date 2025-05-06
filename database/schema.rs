// /home/inno/elights_jobes-research/database/schema.rs
// @generated automatically by Diesel CLI.

diesel::schema! {
    core_schema (DbSchema) {
        // Define tables based on your up.sql migration
        audit_logs (log_id) {
            log_id -> Int8,
            timestamp -> Timestamptz,
            user_id -> Nullable<Uuid>,
            actor_identifier -> Varchar,
            action -> Varchar,
            target_type -> Nullable<Varchar>,
            target_id -> Nullable<Varchar>,
            outcome -> Varchar,
            details -> Nullable<Jsonb>,
            error_message -> Nullable<Text>,
        }

        transactions (transaction_id) {
            transaction_id -> Uuid,
            debit_wallet_id -> Nullable<Uuid>,
            credit_wallet_id -> Nullable<Uuid>,
            transaction_type -> Varchar,
            status -> Varchar,
            amount -> Numeric,
            currency_code -> Varchar,
            description -> Nullable<Text>,
            external_ref_id -> Nullable<Varchar>,
            metadata -> Nullable<Jsonb>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
            settlement_at -> Nullable<Timestamptz>,
        }

        users (user_id) {
            user_id -> Uuid,
            username -> Varchar,
            email -> Varchar,
            password_hash -> Text,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }

        wallets (wallet_id) {
            wallet_id -> Uuid,
            user_id -> Uuid,
            wallet_type -> Varchar,
            currency_code -> Varchar,
            balance -> Numeric,
            bank_name -> Nullable<Varchar>,
            account_number_hash -> Nullable<Text>,
            iban_hash -> Nullable<Text>,
            bic_swift -> Nullable<Varchar>,
            routing_number_hash -> Nullable<Text>,
            address -> Nullable<Varchar>,
            address_index -> Nullable<Int4>,
            status -> Varchar,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }
}

// Define relationships between tables
diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(transactions -> wallets (credit_wallet_id)); // Specify foreign key column name if needed
// diesel::joinable!(transactions -> wallets (debit_wallet_id)); // Diesel doesn't easily support multiple FKs to same table by default, often handled in queries
diesel::joinable!(wallets -> users (user_id));

// Allow tables to appear in the same query (optional but often helpful)
diesel::allow_tables_to_appear_in_same_query!(
    audit_logs,
    transactions,
    users,
    wallets,
);