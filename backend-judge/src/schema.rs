// @generated automatically by Diesel CLI.

diesel::table! {
    contests (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 2000]
        description -> Varchar,
        num_problems -> Int4,
        start_date -> Timestamp,
        end_date -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        creator_id -> Int4,
    }
}

diesel::table! {
    problems (contest_id, problem_num) {
        #[max_length = 255]
        name -> Varchar,
        problem_num -> Int4,
        num_tests -> Int4,
        #[max_length = 255]
        contest_id -> Varchar,
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

diesel::joinable!(contests -> users (creator_id));
diesel::joinable!(problems -> contests (contest_id));

diesel::allow_tables_to_appear_in_same_query!(
    contests,
    problems,
    users,
);
