mod config;

use std::fs;

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

    let sql_file_path = "src/querys/20211013120000_drop_all_table.sql";
    let sql_query = fs::read_to_string(sql_file_path)?;

    // SQLクエリを実行
    sqlx::query(&sql_query).execute(&pool).await?;

    println!("SQL query from file executed successfully");

    Ok(())
}
