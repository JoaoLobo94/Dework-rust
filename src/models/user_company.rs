pub mod UserCompany {
	#[derive(Queryable, Debug)]
	pub struct UserCompany {
	    pub id: i32,
	    pub company_id: i32,
	    pub user_id: i32,
	}
    }