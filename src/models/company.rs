#[derive(Queryable, Debug)]
pub struct Company {
	pub id: i32,
	pub name: String,
	pub github: String,
	pub private_key: String,
	pub balance: Option<i32>,
	pub created_at: NaiveDateTime,
    }