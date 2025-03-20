// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        body -> Text,
    }
}
