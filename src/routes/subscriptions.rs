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
    request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name = %form.name,
  )
)]
pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!("Adding a new subscriber",%request_id, subscriber_email = %form.email, subscriber_name = %form.name);

    let _request_span_guard = request_span.enter();

    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );

    let query_span = tracing::info_span!("Saving new subscriber details in database");

    match query!(
        r#"
        INSERT INTO subscriptions (id, email, names, subscribed_at)
        VALUES ($1, $2, $3, $4)
      "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(error) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                error
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
