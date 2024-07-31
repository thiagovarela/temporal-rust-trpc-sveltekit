use sqlx::Executor;
use sqlx::{postgres::PgPoolOptions, PgPool};


pub async fn database_pool(db_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(15)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                conn.execute("SET application_name = 'gabo'; SET search_path = 'public';")
                    .await?;
                Ok(())
            })
        })
        .connect(db_url)
        .await
        .expect("Can't connect to database")
}