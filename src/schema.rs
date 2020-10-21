table! {
    todos (id) {
        id -> Varchar,
        user_id -> Varchar,
        content -> Text,
        checked -> Bool,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(todos -> users (user_id));

allow_tables_to_appear_in_same_query!(todos, users,);
