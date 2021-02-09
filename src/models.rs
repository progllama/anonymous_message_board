#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub text: String,
    pub published: chrono::NaiveDateTime,
}