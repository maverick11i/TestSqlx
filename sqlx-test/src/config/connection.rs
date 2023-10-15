use anyhow::Result;
use sqlx::PgPool;
use std::env;

// 接続情報を格納する構造体
pub struct ConnectionDatabase {
    host: String,
    port: String,
    name: String,
    user: String,
    pass: String,
}

impl ConnectionDatabase {
    // 構造体のインスタンスを生成するメソッド
    fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        // .env ファイルから環境変数を読み込む
        let host = env::var("DB_HOST")?;
        let port = env::var("DB_PORT")?;
        let name = env::var("DB_NAME")?;
        let user = env::var("DB_USER")?;
        let pass = env::var("DB_PASS")?;

        Ok(ConnectionDatabase {
            host,
            port,
            user,
            pass,
            name,
        })
    }

    // 接続文字列を生成するメソッド
    fn build_db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.name
        )
    }

    // データベースへの接続を確立するメソッド
    pub async fn connect(&self) -> Result<PgPool> {
        let db_url = self.build_db_url();
        let pool = PgPool::connect(&db_url).await?;
        Ok(pool)
    }
}

impl Default for ConnectionDatabase {
    fn default() -> Self {
        Self::new().expect("Failed to create ConnectionDatabase instance") // Resultのunwrap修正
    }
}
