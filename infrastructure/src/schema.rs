// @generated automatically by Diesel CLI.

diesel::table! {
    users (name) {
        #[max_length = 255]
        name -> Varchar,
        age -> Int4,
    }
}
