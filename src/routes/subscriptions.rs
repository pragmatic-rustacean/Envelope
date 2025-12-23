#![allow(unused)]

use sqlx::types::chrono::Utc;
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
    match insert_subscriber(&form, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new subscriber's details ", skip(form, pool))]
async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO subscriptions (id, email, names, subscribed_at)
        VALUES ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        form.email,
        form.name,
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

/*
  Returns 'true' if the provided input passes all our validation constraints, on subscribers 'name(s)', otherwise 'false'.
*/
pub fn is_valid_name(name: &str) -> &str {
    let clean_name = name.trim().is_empty();
    unimplemented!("Fix me, love...");
}
