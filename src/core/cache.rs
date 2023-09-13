use std::env;
use std::marker::PhantomData;

use anyhow::{Context, Result};
use redis::Client;

#[derive(Debug)]
pub struct Disconnected;

#[derive(Debug)]
pub struct Connected;

#[derive(Debug)]
pub struct CacheState<T = self::Disconnected> {
    conn: Client,
    state: PhantomData<T>,
}

impl CacheState<self::Disconnected> {
    pub async fn connect() -> Result<CacheState<self::Connected>> {
        // Get the environment variables.
        let host = env::var("DRAGONFLY_HOST").with_context(|| "DRAGONFLY_HOST is not set")?;
        let port = env::var("DRAGONFLY_PORT").with_context(|| "DRAGONFLY_PORT is not set")?;
        let pass = env::var("DRAGONFLY_PASS").with_context(|| "DRAGONFLY_PASS is not set")?;

        // Connect to the database.
        let client = Client::open(format!("redis://:{}@{}:{}", pass, host, port))?;

        // simulate connection
        let _ = client.get_async_connection().await?;

        Ok(CacheState {
            conn: client,
            state: PhantomData,
        })
    }
}

impl CacheState<self::Connected> {
    pub fn get_new_connection(&self) -> Client {
        self.conn.clone()
    }
}
