#[derive(Queryable, Debug)]
pub struct Story {
    pub id: i32,
    pub pull_request: String,
    pub type_: String,
    pub merged: Option<bool>,
    pub balance: Option<i32>,
    pub vote_balance: Option<i32>,
    pub company_id: i32,
    pub created_at: NaiveDateTime,
}