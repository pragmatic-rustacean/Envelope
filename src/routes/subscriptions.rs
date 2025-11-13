use sqlx::types::chrono::Utc;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<FormData>, pool: Data<PgPool>) -> HttpResponse {
    let id = Uuid::new_v4();

    match query!(
        r#"
        INSERT INTO subscriptions (id, email, names, subscribed_at)
        VALUES ($1, $2, $3, $4)
      "#,
        id,
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => {
            println!("Failed to execute query: {}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}
