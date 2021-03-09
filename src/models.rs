use super::schema::{comments, posts, users};

#[derive(Insertable, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub user_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub user_id: i32,
}

#[derive(Queryable, Debug)]
pub struct Comment {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub body: &'a str,
    pub user_id: i32,
    pub post_id: i32,
}
