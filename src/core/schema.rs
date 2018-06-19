table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        added -> Timestamp,
        due -> Timestamp,
        list -> Varchar,
        notes -> Text,
        completed -> Bool,
        priority -> Varchar,
    }
}
