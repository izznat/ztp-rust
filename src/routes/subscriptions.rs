use actix_web::{HttpResponse, web};
use sqlx::SqlitePool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Adding a new subscriber.", skip(form, pool), fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<SqlitePool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database.",
    skip(form, pool)
)]
async fn insert_subscriber(pool: &SqlitePool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            insert into subscriptions (email, name)
            values ($1, $2)
            "#,
        form.email,
        form.name,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
