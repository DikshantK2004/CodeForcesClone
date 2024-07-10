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
    problems (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        problem_num -> Int4,
        num_tests -> Int4,
        num_samples -> Int4,
        #[max_length = 255]
        contest_id -> Varchar,
        time_limit -> Int4,
        accepted -> Int4,
    }
}

diesel::table! {
    submissions (id) {
        id -> Int4,
        #[max_length = 6000]
        code -> Varchar,
        #[max_length = 20]
        extension -> Varchar,
        user_id -> Int4,
        problem_id -> Int4,
        created_at -> Timestamp,
        #[max_length = 255]
        verdict -> Varchar,
        time_taken -> Nullable<Int4>,
    }
}

diesel::table! {
    test_results (id) {
        id -> Int4,
        submission_id -> Int4,
        test_num -> Int4,
        #[max_length = 1500]
        out -> Varchar,
        #[max_length = 40]
        verdict -> Varchar,
        time_taken -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        verified -> Bool,
    }
}

diesel::joinable!(contests -> users (creator_id));
diesel::joinable!(problems -> contests (contest_id));
diesel::joinable!(submissions -> problems (problem_id));
diesel::joinable!(submissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contests,
    problems,
    submissions,
    test_results,
    users,
);
