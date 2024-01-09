// @generated automatically by Diesel CLI.

diesel::table! {
    deployments (id) {
        id -> Int4,
        p -> Text,
        op -> Text,
        tick -> Text,
        max -> Text,
        lim -> Text,
        input_data -> Text,
        minted -> Text,
        holders -> Int8,
        trx_hash -> Text,
        chain_id -> Int8,
        from_address -> Text,
        to_address -> Text,
        height -> Int8,
        timestamp -> Int8,
    }
}

diesel::table! {
    mints (id) {
        id -> Int4,
        p -> Text,
        op -> Text,
        tick -> Text,
        tick_id -> Text,
        amt -> Text,
        input_data -> Text,
        trx_hash -> Text,
        chain_id -> Int8,
        from_address -> Text,
        to_address -> Text,
        height -> Int8,
        timestamp -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(deployments, mints,);
