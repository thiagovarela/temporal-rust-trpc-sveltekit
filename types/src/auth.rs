use chrono::{ DateTime, Utc };
use typeshare::typeshare;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,    
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}
#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCredentials {
    pub user_id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignUpInput {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[typeshare]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginWithEmailInput {
    pub email: String,
    pub password: String,
}