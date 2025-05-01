// @generated automatically by Diesel CLI.
// /home/inno/elights_jobes-research/database/schema.rs

diesel::table! {
    core_schema.accounts (id) {
        id -> Int4,
        owner_username -> Text,
        account_identifier -> Text,
        account_type -> Text,
        currency -> Text,
        balance -> Numeric,
        bank_name -> Nullable<Text>,
        routing_number -> Nullable<Text>,
        iban -> Nullable<Text>,
        bic_swift -> Nullable<Text>,
        crypto_address -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    core_schema.transactions (id) {
        id -> Uuid,
        debit_account_id -> Nullable<Int4>,
        credit_account_id -> Nullable<Int4>,
        amount -> Numeric,
        currency -> Text,
        transaction_type -> Text,
        status -> Text,
        description -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    core_schema.users (username) {
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (owner_username));
diesel::joinable!(transactions (credit_account_id) -> accounts (id));
diesel::joinable!(transactions (debit_account_id) -> accounts (id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
    users,
);