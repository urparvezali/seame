use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPayload {
    pub username: String,
    pub password: String,
    pub email: String,
    pub birthday: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostPayload {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdatePasswordPayload {
	pub password: String,
	pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdateBirthdayPayload {
	pub birthday: NaiveDate,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdateEmailPayload {
	pub email: String,
}
