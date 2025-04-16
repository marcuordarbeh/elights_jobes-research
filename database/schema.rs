// database/schema.rs

diesel::table! {
    core_schema.users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::table! {
    core_schema.accounts (id) {
        id -> Int4,
        user_id -> Int4,
        account_number -> Varchar,
        routing_number -> Varchar,
        bank_name -> Varchar,
    }
}

diesel::table! {
    core_schema.transactions (id) {
        id -> Int4,
        account_id -> Int4,
        amount -> Numeric,
        currency -> Varchar,
        transaction_type -> Varchar,
        status -> Varchar,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(transactions -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    accounts,
    transactions,
);
