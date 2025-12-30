#![allow(unused)]

use crate::domain::{
    new_subscriber::{NewSubscriber},
    subscriber_email::SubscriberEmail,
    subscriber_name::SubscriberName,
};
use sqlx::types::chrono::Utc;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[tracing::instrument (
  name = "Adding a new subscriber",
  skip(form, pool),
  fields (
    subscriber_email = %form.email,
    subscriber_name = %form.name,
  )
)]
pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };

    match insert_subscriber(&new_subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new subscriber's details ", skip(new_subscriber, pool))]
async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, names, subscribed_at)
        VALUES ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {}", e);
        e
    })?;
    Ok(())
}
