/// Populate the database with some data.
use diesel::prelude::*;

fn main() {
    use hello::models::*;
    use hello::schema::posts::dsl::*;
    use hello::schema::users::dsl::*;
    use hello::schema::{comments, posts, users};

    let insert_users = vec![NewUser { name: "John" }, NewUser { name: "Jane" }];

    let conn = hello::establish_connection();
    diesel::insert_into(users::table)
        .values(&insert_users)
        .execute(&conn)
        .unwrap();

    let all_users = users.load::<User>(&conn).unwrap();

    let insert_posts = vec![
        NewPost {
            title: "A",
            body: Some("aaa"),
            user_id: all_users[0].id,
        },
        NewPost {
            title: "B",
            body: None,
            user_id: all_users[0].id,
        },
        NewPost {
            title: "C",
            body: None,
            user_id: all_users[1].id,
        },
    ];

    diesel::insert_into(posts::table)
        .values(&insert_posts)
        .execute(&conn)
        .unwrap();

    let all_posts = posts.load::<Post>(&conn).unwrap();

    let insert_comments = vec![
        NewComment {
            body: "A",
            user_id: all_users[1].id,
            post_id: all_posts[0].id,
        },
        NewComment {
            body: "B",
            user_id: all_users[0].id,
            post_id: all_posts[0].id,
        },
        NewComment {
            body: "C",
            user_id: all_users[1].id,
            post_id: all_posts[2].id,
        },
        NewComment {
            body: "D",
            user_id: all_users[0].id,
            post_id: all_posts[2].id,
        },
    ];

    diesel::insert_into(comments::table)
        .values(&insert_comments)
        .execute(&conn)
        .unwrap();
}
