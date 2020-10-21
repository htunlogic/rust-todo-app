table! {
  users_todos (id) {
      id -> Varchar,
      email -> Varchar,
      content -> Varchar,
      checked -> Bool,
      user_id -> Varchar,
  }
}
