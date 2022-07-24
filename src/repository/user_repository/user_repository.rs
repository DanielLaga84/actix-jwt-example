use crate::models::user::{Login, User, Claims, Register};
use crate::models::response::{LoginResponse, Response};
use mongodb::Client;
use std::path::Path;
use mongodb::error::Error;
use crate::config::{Config, IConfig};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};
use chrono::{Utc, DateTime, Duration};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub trait IUserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error>;
    fn login(&self, login: Login) -> Result<LoginResponse, Response>;
    fn register(&self, user: Register) -> Response;
    fn user_information(&self, token: &str) -> Result<Option<User>, Response>;
    fn protected_function(&self) -> bool;
}
pub struct UserRepository {
    pub connection: &'static Client
}

impl IUserRepository for UserRepository { 
    




}