table! {
    category (id) {
        id -> Int8,
        image_id -> Nullable<Varchar>,
        name -> Varchar,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    post (id) {
        id -> Int8,
        profile_id -> Int8,
        category_id -> Int8,
        title -> Varchar,
        resource_id -> Varchar,
        description -> Nullable<Text>,
        approved_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    profile (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Varchar,
        avatar_id -> Nullable<Varchar>,
        birthday -> Nullable<Timestamptz>,
        email_confirmed_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

joinable!(post -> category (category_id));
joinable!(post -> profile (profile_id));

allow_tables_to_appear_in_same_query!(
    category,
    post,
    profile,
);
