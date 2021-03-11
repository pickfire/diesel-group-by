Diesel group by experiment
==========================

Previous: https://github.com/pickfire/diesel-upsert

This time it requires diesel-git which isn't released.

I was trying out `group_by` with `count` in diesel. Originally I was trying it
on a single table, I wanted to find how many comments are there for each user
and post. I am able to `group_by` and get the items out easily.

I was a bit stuck at first getting some issues on an error getting count to
load as `u32`. But after I took a look at the example for `group_by` and I tried
searching the type `BigInt`, I saw a big `ToSql` and `FromSql` impls at the
front, even though the error messages are not that good I can at least find it
easily through the docs, at least it took me some time to figure this out.
Changing it to `i32` works but I wonder why it wasn't `u32` for count?

```rust
error[E0277]: the trait bound `(i32, i32, u64): FromStaticSqlRow<(diesel::sql_types::Integer, diesel::sql_types::Integer, BigInt), Sqlite>` is not satisfied
  --> src/main.rs:20:44
   |
20 |     let data: Vec<(i32, i32, u64)> = query.load(&conn).unwrap();
   |                                            ^^^^ the trait `FromStaticSqlRow<(diesel::sql_types::Integer, diesel::sql_types::Integer, BigInt), Sqlite>` is not implemented for `(i32, i32, u64)`
   |
   = help: the following implementations were found:
             <(B, C, A) as FromStaticSqlRow<(SB, SC, SA), __DB>>
   = note: required because of the requirements on the impl of `Queryable<(diesel::sql_types::Integer, diesel::sql_types::Integer, BigInt), Sqlite>` for `(i32, i32, u64)`
   = note: required because of the requirements on the impl of `FromSqlRow<(diesel::sql_types::Integer, diesel::sql_types::Integer, BigInt), Sqlite>` for `(i32, i32, u64)`
   = note: required because of the requirements on the impl of `LoadQuery<SqliteConnection, (i32, i32, u64)>` for `SelectStatement<hello::schema::comments::table, diesel::query_builder::select_clause::SelectClause<(hello::schema::comments::user_id, post_id, diesel::expression::count::count::count<diesel::sql_types::Integer, hello::schema::comments::id>)>, diesel::query_builder::distinct_clause::NoDistinctClause, diesel::query_builder::where_clause::NoWhereClause, diesel::query_builder::order_clause::NoOrderClause, LimitOffsetClause<NoLimitClause, NoOffsetClause>, diesel::query_builder::group_by_clause::GroupByClause<(hello::schema::comments::user_id, post_id)>>`
```

I also tried switching the order of `group_by` and `select` and I was
surprised now that I get a compile-time error, I will just send a
pull request for this to mention it in the docs.

Then I was trying out I want to get the user name and post title instead of
the user id and post id from the column name. I tried the query in sqlite3
and it worked but I just realized that it does not work for others because
I think I recall that it is required fo the field to be within the select.

```sh
$ sqlite3 target/test.db 'select posts.title, users.name, count(comments.id) from comments inner join users on comments.user_id = users.id inner join posts on comments.post_id = posts.id group by comments.user_id, post_id;'
```

I didn't get to test group by with `(users::name, posts::title)` because that
is not what I want (it could be the same but different id) but I went and try
it out, with the help of
`allow_columns_to_appear_in_same_group_by_clause!(users::name, posts::title)`
and indeed it does work.

So far, I learned that diesel requires some macro to add linkage between
relationship for schema, if I happen to miss it I may face one of the errors
like seen above. Not sure if this could be improved but there seemed to be
lots of different macro for different stuff, I wonder if it would be easier
to just have a macro to define the relationship and we don't need the rest.

When I went back to trying to use a subquery to do the aggregation and use
the part of subquery as `FROM`, then I got stuck not sure how can we get
`AS` in diesel. I was stuck on this part so I tried getting help from the
diesel gitter channel.

```sql
SELECT users.name, posts.title, agg.count
 FROM (
    SELECT comments.user_id, comments.post_id, count(comments.id) AS count
     FROM comments
    GROUP BY comments.user_id, comments.post_id
) as agg
INNER JOIN users ON users.id = agg.user_id
INNER JOIN posts ON posts.id = agg.post_id;
```

I found out that the error is showing something else but the maintainer
pointed out that the incorrect part was that I was using `i32` rather than
`i64` for `BigInt`. After I fixed that I got it compiling again.

```rust
error[E0277]: the trait bound `CommentCount: FromSqlRow<Untyped, _>` is not satisfied
  --> src/main.rs:48:9
   |
48 |     "#).get_results::<CommentCount>(&conn).unwrap();
   |         ^^^^^^^^^^^ the trait `FromSqlRow<Untyped, _>` is not implemented for `CommentCount`
   |
   = note: required because of the requirements on the impl of `LoadQuery<_, CommentCount>` for `SqlQuery`

error: aborting due to previous error; 2 warnings emitted
```

Other than the confusing error, if I didn't face that it should be very
straightforward if I found out about `QueryableByName` derive.

The example I tested out here have 3 struct (based on the previous ones).
sqlite was used for easy testing.

    +---------+     +---------+     +---------+
    | User    |<-+  | Post    |     | Comment |
    +---------+  |  +---------+     +---------+
    | id      |  |  | id      |     | id      |
    | name    |  |  | title   |     | body    |
    |         |  |  | body    |<----+ post_id |
    |         |  |  | user_id +--+--+ user_id |
    +---------+  |  +---------+  |  +---------+
                 +---------------+

I want to find all posts that does not have any comments. The query,

```rust
#[derive(QueryableByName, Debug)]
struct CommentCount {
    #[sql_type = "Text"]
    name: String,
    #[sql_type = "Text"]
    title: String,
    #[sql_type = "BigInt"]
    count: i64,
}

let data = diesel::sql_query(r#"
SELECT users.name, posts.title, agg.count AS count
 FROM (
    SELECT comments.user_id, comments.post_id, count(comments.id) AS count 
     FROM comments 
    GROUP BY comments.user_id, comments.post_id 
) as agg                       
INNER JOIN users ON users.id = agg.user_id
INNER JOIN posts ON posts.id = agg.post_id;
"#).get_results::<CommentCount>(&conn).unwrap();
```

## Get started

Rust, diesel_cli (with `sqlite` feature) is required.

```
$ diesel migration run
$ cargo run --bin init  # populate database
$ cargo run --bin hello  # multi join query
```
