use sqlx::postgres::PgPoolOptions;

use std::env;

#[derive(Debug)]
struct PostgresConfig {
    host: String,
    user: String,
    password: String,
    db: String,
    port: u32,
}

fn get_env_var(var_name: &str) -> String {
    let val = match env::var(var_name) {
        Ok(val) => val,
        Err(err) => panic!("Error {} {:?}", var_name, err),
    };
    assert!(!val.is_empty());
    val
}

fn get_postgres_cfg() -> PostgresConfig {
    let host: String = get_env_var("POSTGRES_HOSTNAME");
    let password: String = get_env_var("POSTGRES_PASSWORD");
    let user: String = get_env_var("POSTGRES_USER");
    let port: u32 = get_env_var("POSTGRES_PORT").parse().expect("Invalid POSTGRES_PORT env variable.");
    let db: String = get_env_var("POSTGRES_DB");
    PostgresConfig {
        host,
        user,
        password,
        db,
        port
    }
}

use futures::TryStreamExt;
use sqlx::{Row, Pool, Postgres, Executor};

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    let config = get_postgres_cfg();
    let connection_string = format!("postgres://{}:{}@{}:{}/{}", config.user, config.password, config.host, config.port, config.db);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string).await?;

    /* Fetch tables example */
    // let mut table_names = pool.fetch("SELECT * FROM pg_catalog.pg_tables WHERE schemaname = 'public';");
    //
    // while let Some(row) = table_names.try_next().await? {
    //     // map the row into a user-defined domain type
    //     let table: &str = row.try_get("tablename")?;
    //     println!("{}", table);
    // }

    println!("Successfully connected to the database at: {}", connection_string);


    Ok(pool)
}