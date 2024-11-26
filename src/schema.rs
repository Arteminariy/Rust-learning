diesel::table! {
    users {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        is_married -> Bool,
    }
}