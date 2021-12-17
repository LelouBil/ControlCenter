table! {
    postes (ip) {
        is_on -> Bool,
        is_compromised -> Bool,
        os -> Text,
        hostname -> Text,
        ip -> Text,
    }
}

table! {
    users (username) {
        username -> Text,
        password -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    postes,
    users,
);
