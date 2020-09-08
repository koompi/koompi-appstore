use async_graphql::ID;
use bson::{self, oid::ObjectId};
use serde_derive::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

// GQL Struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: bool,
}

// Mongo Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: bool,
}

// Base implemenation
impl User {
    pub fn new() -> Self {
        User {
            id: ID::from(""),
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            status: false,
        }
    }
}

// Mongo implementation
impl UserModel {
    pub fn new() -> UserModel {
        let converted_id = bson::oid::ObjectId::new();
        UserModel {
            _id: converted_id,
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            status: false,
        }
    }
    pub fn to_norm(&self) -> User {
        User {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            status: self.status.to_owned(),
        }
    }
}

// GraphQL
#[async_graphql::Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn password(&self) -> &str {
        &self.password
    }
    async fn status(&self) -> &bool {
        &self.status
    }
}
