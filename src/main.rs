use sqlx::postgres::PgPool;
use std::env;
use std::fmt;
use anyhow;
use std::fs::File;
use std::io::Write;

const FILE_PATH: &str = "/home/arturcs/Documents/recados_backup.txt";

#[derive(Debug)]
struct Recado {
    id: i32,
    remetente: Option<String>,
    conteudo: Option<String>
}

impl fmt::Display for Recado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rem: &str = match &self.remetente {
            Some(r) => r, 
            None => "Ninguem"
        };
        let con: &str = match &self.conteudo {
            Some(con) => con, 
            None => "Nada"
        };

        write!(f, "id: {}, remetente: {}, conteudo: {}", self.id, rem, con)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    save_to_file(
        &all_recados(&pool)
        .await?
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join("\n")
    )?;

    Ok(())
}

async fn all_recados(con_pool: &PgPool) -> anyhow::Result<Vec<Recado>> {
    Ok(sqlx::query_as!(Recado,
        r#"
        SELECT id, remetente, conteudo
        FROM recados 
        ORDER BY id
        "#
    ).fetch_all(con_pool).await?)
}

fn save_to_file(content: &str) -> anyhow::Result<()>{
    let mut file = File::create(FILE_PATH)?;
    Ok(file.write_all(content.as_bytes())?)
}
