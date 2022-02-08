table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        github -> Varchar,
        private_key -> Varchar,
        balance -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

table! {
    contributions (id) {
        id -> Int4,
        pull_request -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        merged -> Nullable<Bool>,
        balance -> Nullable<Int4>,
        vote_balance -> Nullable<Int4>,
        story_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    stories (id) {
        id -> Int4,
        pull_request -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        merged -> Nullable<Bool>,
        balance -> Nullable<Int4>,
        vote_balance -> Nullable<Int4>,
        company_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    user_companies (id) {
        id -> Int4,
        company_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    user_contributions (id) {
        id -> Int4,
        company_id -> Int4,
        contribution_id -> Int4,
        balance -> Nullable<Int4>,
        vote_balance -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        public_key -> Varchar,
        job -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(contributions -> companies (story_id));
joinable!(stories -> companies (company_id));
joinable!(user_companies -> companies (company_id));
joinable!(user_companies -> users (user_id));
joinable!(user_contributions -> companies (company_id));
joinable!(user_contributions -> contributions (contribution_id));

allow_tables_to_appear_in_same_query!(
    companies,
    contributions,
    stories,
    user_companies,
    user_contributions,
    users,
);
