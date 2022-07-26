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

    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let _config: Config = Config {};
        let database_name = _config.get_config_with_key("DATABASE_NAME");
        let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
        let db = self.connection.database(database_name.as_str());
        let cursor = db.collection(collection_name.as_str()).find_one(doc! {"email": email}, None).unwrap();
        match cursor {
            Some(doc) => {
                match bson::from_bson(bson::Bson::Document(doc)) {
                    Ok(model) => Ok(model),
                    Err(e) => Err(Error::from(e))
                }
            }
            None => Ok(None)
        }

    }

    fn login(&self, user: Login) -> Result<LoginResponse, Response> {
     match self.find_user_with_email(user.email.to_string()).unwrap() {
        Some(x) => {
            let mut sha = Sha256::new();
            sha.input_str(user.password.as_str());
            if x.password == sha.result_str() {
                //JWT implementation
                let _config: Config = Config {};
                let _var = _config.get_config_with_key("SECRET_KEY");
                let key =  _var.as_bytes();
                
                let mut _date: DateTime<Utc>;

                // REMEMBER ME implementation
                if !user.remember_me {
                    _date = Utc::now() + Duration::hours(1);
                } else {
                    _date =  Utc::now() + Duration::days(365);
                }

                let my_claims = Claims { sub: user.email, exp: _date.timestamp() as usize};
                let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)).unwrap();
                Ok(LoginResponse{
                    status: true,
                    token: token.to_string(),
                    message: "Login success!".to_string(),

                })
            } else {
                Err(Response { status : false, message: "Check your user informations.".to_string()})
            }

        }
        None => {
            Err(Response { status : false, message: "Check your user informations.".to_string()})
        }

     }
    }

    fn register(&self, user: Register) -> Response {
        let _exist = self.find_user_with_email((&user.email).parse().unwrap()).unwrap();
        match _exist { 
            Some(_) => Response { message: "This e-mail is already taken, use another e-mail".to_string(), status:false},
            None => {
                let _config: Config = Config{};
                let database_name = _config.get_config_with_key("DATABASE_NAME");
                let collection_name = _config.get_config_with_key("USER_COLLECTION_NAME");
                let db = self.connection.database(database_name.as_str());
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());
                let hash_pw = sha.result_str();
                let user_id = uuid::Uuid::new_v4().to_string();
                let _ex = db.collection(collection_name.as_str()).insert_one(doc! {"user_id": user_id, "name": user.name, "surname": user.surname, "email": user.email, "password": hash_pw, "phone": user.phone, "birth_date": user.birth_date }, None);
                match _ex {
                    Ok(_) => Response { status: true, message : "Registered.".to_string()},
                    Err(_) => Response { status: false, message : "Something went wrong, please try again.".to_string()}
                }
        }}
    }

    fn user_information(&self, token: &str) -> Result<Option<User>, Response> {
        let _config: Config = Config {};
        let _var = _config.get_config_with_key("SECRET_KEY");
        let key = _var.as_bytes();
        let _decode = decode::<Claims>(token.as_ref(), &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS256));
        match _decode {
            Ok(decoded) => {
                match self.find_user_with_email((decoded.claims.sub.to_string()).parse().unwrap()) {
                    Ok(user) => {
                        Ok(user)
                    },
                    Err(_) => Err(Response { status: false, message: "Something went wrong, please try again.".to_string()})
                }
            }
            Err(_) => Err(Response { status: false, message: "Invalid Token".to_string() }) 
        }
    }

    fn protected_function(&self) -> bool {
        true
    }




}