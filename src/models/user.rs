use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User { 
    pub user_id: String,
    pub name: String,
    pub surname: String,
    pub phone: Option<String>,
    pub email: String,
    pub password: String,
    pub birth_date: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
pub sub: String,
pub exp: usize
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub birth_date: String
}