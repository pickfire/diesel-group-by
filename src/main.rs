use diesel::prelude::*;
use diesel::dsl::count;

fn main() {
    // use hello::models::User;
    use hello::schema::{comments, posts, users};

    let conn = hello::establish_connection();

    let subquery = comments::table
        .group_by((comments::user_id, comments::post_id))
        .select((comments::user_id, comments::post_id, count(comments::id)));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&subquery).to_string()
    );
    let query = subquery
        .inner_join(posts::table)
        .inner_join(users::table)
        .select((users::name, posts::title, count(comments::id)));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    let data: Vec<(String, String, i32)> = query.load(&conn).unwrap();

    // dbg!(data);
}
