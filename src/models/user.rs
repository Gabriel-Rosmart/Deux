use mongodb::{
    bson::{doc, Document},
    error::Result as MongoResult,
    results::{InsertOneResult, DeleteResult},
    Collection,
};

use serde::{Deserialize, Serialize};

use crate::crypto::hash::{ Hash, Iterations };

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(
        collection: &Collection<Document>,
        (email, password): (&str, &str),
    ) -> MongoResult<InsertOneResult> {
        let hash = Hash::generate(password, Iterations::MEDIUM);

        let result = collection
            .insert_one(
                doc! {
                    "email": email,
                    "password": hash
                },
                None,
            )
            .await?;

        Ok(result)
    }

    pub async fn delete(collection: &Collection<Document>, email: &str) -> MongoResult<DeleteResult> {
        let result = collection.delete_one(doc! {
            "email": email
        }, None).await?;
        Ok(result)
    }

    pub async fn verify(
        collection: &Collection<Self>,
        (email, password): (&str, &str),
    ) -> MongoResult<bool> {
        /* Find giver user, if not found return after checking password */

        let user = collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await?;

        if user.is_none() {
            return Ok(false);
        }

        Ok(Hash::verify(password, &user.unwrap().password))
    }

    pub async fn exists(collection: &Collection<Document>, email: &str) -> MongoResult<bool> {
        let user = collection.find_one(doc! {
            "email": email
        }, None).await?;

        Ok(user.is_some())
    }
}
