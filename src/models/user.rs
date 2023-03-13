use mongodb::{
    bson::{doc, Document, oid::ObjectId},
    error::Result as MongoResult,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use serde::{Deserialize, Serialize};

use crate::{crypto::hash::{Hash, Iterations}, extractors::profile::UpdateProfileRequest};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    #[serde(skip_serializing)]
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

    pub async fn update(
        collection: &Collection<Document>,
        email: &str,
        fields: UpdateProfileRequest,
    ) -> MongoResult<UpdateResult> {

        let hash = Hash::generate(&fields.password, Iterations::MEDIUM);

        let result = collection
            .update_one(doc! {
                "email": email
            }, doc! {
                "$set": {
                    "password": hash
                }
            }, None).await?;

        Ok(result)
    }

    pub async fn delete(
        collection: &Collection<Document>,
        email: &str,
    ) -> MongoResult<DeleteResult> {
        let result = collection
            .delete_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await?;
        Ok(result)
    }

    pub async fn verify(
        collection: &Collection<Self>,
        (email, password): (&str, &str),
    ) -> MongoResult<(bool, Option<ObjectId>)> {
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
            return Ok((false, None));
        }

        let user = user.unwrap();

        Ok((Hash::verify(password, &user.password), Some(user._id)))
    }

    pub async fn exists(collection: &Collection<Document>, email: &str) -> MongoResult<bool> {
        let user = collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await?;

        Ok(user.is_some())
    }
}
