#[derive(Queryable, Debug)]
pub struct UserContribution {
    pub id: i32,
    pub company_id: i32,
    pub contribution_id: i32,
    pub balance: Option<i32>,
    pub vote_balance: Option<i32>,
}
