// @generated automatically by Diesel CLI.

diesel::table! {
    guestbooks (id) {
        id -> Integer,
        name -> Text,
        message -> Text,
    }
}
