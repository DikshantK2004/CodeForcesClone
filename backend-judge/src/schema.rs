// @generated automatically by Diesel CLI.

diesel::table! {
    contest (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        start_date -> Timestamp,
        end_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        creator_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        verified -> Bool,
    }
}

diesel::joinable!(contest -> users (creator_id));

diesel::allow_tables_to_appear_in_same_query!(
    contest,
    users,
);
