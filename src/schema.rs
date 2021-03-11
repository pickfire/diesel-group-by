table! {
    comments (id) {
        id -> Integer,
        body -> Text,
        post_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(comments, posts, users,);
