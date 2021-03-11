use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};

fn main() {
    // use hello::models::User;
    // use hello::schema::{comments, posts, users};

    let conn = hello::establish_connection();

    // let subquery = comments::table
    //     .group_by((comments::user_id, comments::post_id))
    //     .select((comments::user_id, comments::post_id, count(comments::id)));
    // println!(
    //     "{}",
    //     diesel::debug_query::<diesel::sqlite::Sqlite, _>(&subquery).to_string()
    // );
    // let query = subquery
    //     .inner_join(posts::table)
    //     .inner_join(users::table)
    //     .select((users::name, posts::title, count(comments::id)));
    // println!(
    //     "{}",
    //     diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    // );
    // let data: Vec<(String, String, i32)> = query.load(&conn).unwrap();

    #[derive(QueryableByName, Debug)]
    struct CommentCount {
        #[sql_type = "Text"]
        name: String,
        #[sql_type = "Text"]
        title: String,
        #[sql_type = "BigInt"]
        count: i64,
    }

    let data = diesel::sql_query(
        r#"
SELECT users.name, posts.title, agg.count AS count
 FROM (
    SELECT comments.user_id, comments.post_id, count(comments.id) AS count 
     FROM comments 
    GROUP BY comments.user_id, comments.post_id 
) as agg                       
INNER JOIN users ON users.id = agg.user_id
INNER JOIN posts ON posts.id = agg.post_id;
    "#,
    )
    .get_results::<CommentCount>(&conn)
    .unwrap();

    dbg!(data);
}
