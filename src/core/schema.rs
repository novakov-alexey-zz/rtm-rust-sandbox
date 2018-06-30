table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        added -> Timestamptz,
        due -> Timestamptz,
        list -> Varchar,
        notes -> Text,
        completed -> Bool,
        priority -> Text,
    }
}
