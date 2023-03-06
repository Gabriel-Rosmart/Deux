use mongodb::{
    bson::{doc, Document},
    error::Result as MongoResult,
    results::InsertOneResult,
    Collection,
};

use serde::{Deserialize, Serialize};

use crate::crypto::hash::Hash;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(
        collection: Collection<Document>,
        (email, password): (String, String),
    ) -> MongoResult<InsertOneResult> {
        let hash = Hash::generate(password);

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

    pub async fn verify(
        collection: Collection<Self>,
        (email, password): (String, String),
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

        Ok(Hash::verify(password.as_str(), &user.unwrap().password))
    }
}
