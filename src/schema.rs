// @generated automatically by Diesel CLI.

diesel::table! {
    information (id) {
        id -> Int4,
        body -> Text,
        section -> Text,
        verbosity -> Int4,
    }
}
