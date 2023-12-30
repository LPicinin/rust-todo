use diesel::table;
table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        completed -> Bool,
    }
}