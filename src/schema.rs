diesel::table! {
    traffic (name) {
        name -> Text,
        rx -> Text,
        tx -> Text
    }
}