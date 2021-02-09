use super::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub create_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub text: &'a str,
}