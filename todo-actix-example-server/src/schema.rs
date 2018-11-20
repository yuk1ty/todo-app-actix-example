table! {
    todos (id) {
        id -> Integer,
        title -> Varchar,
        body -> Nullable<Varchar>,
        status -> Bool,
    }
}
