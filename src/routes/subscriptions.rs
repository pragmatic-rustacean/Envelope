#![allow(unused)]

use crate::domain::subscriber_name::{NewSubscriber, SubscriberName};
use sqlx::types::chrono::Utc;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
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
    let name = match SubscriberName::parse_name(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let subscriber = NewSubscriber {
        email: form.0.email,
        name,
    };
    match insert_subscriber(&subscriber, &pool).await {
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
        new_subscriber.email,
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
