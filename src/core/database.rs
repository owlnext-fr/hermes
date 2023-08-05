use std::env;
use std::marker::PhantomData;

use anyhow::{Context, Result};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

/// A struct representing the disconnected state of the database.
#[derive(Debug)]
pub struct Disconnected;

/// A struct representing the connected state of the database.
#[derive(Debug)]
pub struct Connected;

/// A struct containing the database connection.
/// Default state is disconnected.
///
/// This struct is generic over the state of the connection.
#[derive(Debug)]
pub struct DatabaseState<T = Disconnected> {
    conn: Surreal<Client>,
    state: PhantomData<T>,
}

impl DatabaseState<Disconnected> {
    /// Connects to the database and returns a new DatabaseState with the connected state.
    ///
    /// This will return an Err if any of the following environment variables are not set:
    /// - SDB_HOST (the host of the database)
    /// - SDB_PUBLIC_PORT (the public port of the database)
    /// - SDB_USER (the username to use to connect to the database)
    /// - SDB_PASSWORD (the password to use to connect to the database)
    /// - SDB_NAMESPACE (the namespace to use to connect to the database)
    /// - SDB_DB (the database to use to connect to the database)
    pub async fn connect() -> Result<DatabaseState<Connected>> {
        // Get the environment variables.
        let host = env::var("SDB_HOST").with_context(|| "SDB_HOST is not set")?;
        let port = env::var("SDB_PUBLIC_PORT").with_context(|| "SDB_PUBLIC_PORT is not set")?;
        let user = env::var("SDB_USER").with_context(|| "SDB_USER is not set")?;
        let password = env::var("SDB_PASSWORD").with_context(|| "SDB_PASSWORD is not set")?;
        let namespace = env::var("SDB_NAMESPACE").unwrap_or("surreal".to_string());
        let database = env::var("SDB_DB").unwrap_or("surreal".to_string());

        // Connect to the database.
        let conn = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;

        // Sign in to the database.
        conn.signin(Root {
            username: &user,
            password: &password,
        })
        .await?;

        // Use the namespace and database.
        conn.use_ns(&namespace).use_db(&database).await?;

        // Return the new DatabaseState.
        Ok(DatabaseState {
            conn,
            state: PhantomData,
        })
    }
}

impl DatabaseState<Connected> {
    /// Returns a clone of the connection.
    pub fn get_new_connection(&self) -> Surreal<Client> {
        self.conn.clone()
    }
}
