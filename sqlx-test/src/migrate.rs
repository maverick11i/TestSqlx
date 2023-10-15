mod config;

use crate::config::connection::ConnectionDatabase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = ConnectionDatabase::default().connect().await;
    let pool = match pool {
        Ok(p) => p, // 接続が成功した場合、PgPool を取得
        Err(err) => {
            eprintln!("Failed to establish database connection: {:?}", err);
            return Err(err.into()); // エラーがある場合、エラーを返して終了
        }
    };

    // マイグレーションを実行
    sqlx::migrate!("src/migrations").run(&pool).await?;

    println!("Migration completed successfully");

    Ok(())
}
