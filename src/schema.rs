// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        is_married -> Bool,
    }
}
