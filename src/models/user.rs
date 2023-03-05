use mongodb::{
    error::Result  as MongoResult,
    results::InsertOneResult,
    Collection, 
    bson::{ Document, doc },
};

use crate::crypto::hash::Hash;

pub struct User;

impl User {
    pub async fn create(collection: Collection<Document>, (email, password): (String, String)) -> MongoResult<InsertOneResult> {
        let (hash, salt) = Hash::generate(password.as_str());

        let result = collection.insert_one(doc!{
            "email": email,
            "password": hash,
            "salt": salt
        }, None).await?;

        Ok(result)
    }
}