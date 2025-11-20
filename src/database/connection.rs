use sqlx::PgPool;

pub async fn get_connection() -> PgPool {
    PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}
