table! {
    todo (id) {
        id -> Int8,
        title -> Text,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
