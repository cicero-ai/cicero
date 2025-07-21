
use sqlx::{Row, Pool, MySql, Sqlite, Postgres, Database, Executor};
use tokio::runtime::Runtime;
use std::path::Path;
use std::fs;
use crate::Error;

#[derive(Debug)]
pub struct OpusSqlDb<DB: Database> {
    pool: Pool<DB>,
    rt: Runtime,
}

impl OpusSqlDb<MySql> {
    pub fn connect_mysql(url: &str) -> Result<Self, Error> {
        let rt = Runtime::new()
            .map_err(|e| Error::Generic(format!("Runtime init failed: {}", e)))?;
        let pool = rt.block_on(
            Pool::<MySql>::connect(url)
        ).map_err(|e| Error::Generic(format!("MySQL connect failed: {}", e)))?;
        Ok(Self { pool, rt })
    }

    pub fn insert(&self, query: &str, params: &[&str]) -> Result<u64, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        let result = self.rt.block_on(q.execute(&self.pool))
            .map_err(|e| Error::Generic(format!("Query failed: {}", e)))?;
        Ok(result.last_insert_id())
    }
}

impl OpusSqlDb<Sqlite> {
    pub fn connect_sqlite(dbfile: &str) -> Result<Self, Error> {
        fs::create_dir_all(Path::new(dbfile).parent().unwrap())
            .map_err(|e| Error::Generic(format!("Dir creation failed: {}", e)))?;
        let url = format!("sqlite:{}", dbfile);
        let rt = Runtime::new()
            .map_err(|e| Error::Generic(format!("Runtime init failed: {}", e)))?;
        let pool = rt.block_on(
            Pool::<Sqlite>::connect(&url)
        ).map_err(|e| Error::Generic(format!("SQLite connect failed: {}", e)))?;
        Ok(Self { pool, rt })
    }

    pub fn insert(&self, query: &str, params: &[&str]) -> Result<u64, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        let result = self.rt.block_on(q.execute(&self.pool))
            .map_err(|e| Error::Generic(format!("Query failed: {}", e)))?;
        Ok(result.last_insert_rowid() as u64) // Safe cast for insert IDs
    }
}

impl OpusSqlDb<Postgres> {
    pub fn connect_pgsql(url: &str) -> Result<Self, Error> {
        let rt = Runtime::new()
            .map_err(|e| Error::Generic(format!("Runtime init failed: {}", e)))?;
        let pool = rt.block_on(
            Pool::<Postgres>::connect(url)
        ).map_err(|e| Error::Generic(format!("PostgreSQL connect failed: {}", e)))?;
        Ok(Self { pool, rt })
    }

    pub fn insert(&self, query: &str, params: &[&str]) -> Result<u64, Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        // Use fetch_one to get the returned ID
        let row = self.rt.block_on(q.fetch_one(&self.pool))
            .map_err(|e| Error::Generic(format!("Query failed: {}", e)))?;
        let id: i64 = row.try_get("id")
            .map_err(|e| Error::Generic(format!("Failed to get ID: {}", e)))?;
        Ok(id as u64) // Cast to u64
    }
}

impl<DB> OpusSqlDb<DB>
where
    DB: Database,
    for<'c> &'c Pool<DB>: Executor<'c, Database = DB>,
    for<'q> <DB as Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
    for<'q> String: sqlx::Type<DB> + sqlx::Encode<'q, DB>,
{
    pub fn execute(&self, query: &str, params: &[&str]) -> Result<(), Error> {
        let mut q = sqlx::query(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        self.rt.block_on(q.execute(&self.pool))
            .map_err(|e| Error::Generic(format!("Query failed: {}", e)))?;
        Ok(())
    }

    pub fn fetch_one<T>(&self, query: &str, params: &[&str]) -> Result<Option<T>, Error>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
        for<'q> <DB as Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        for<'q> String: sqlx::Type<DB> + sqlx::Encode<'q, DB>,
    {
        let mut q = sqlx::query_as::<_, T>(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        let result = self.rt.block_on(q.fetch_optional(&self.pool))
            .map_err(|e| Error::Generic(format!("Fetch one failed: {}", e)))?;
        Ok(result)
    }

    pub fn fetch_all<T>(&self, query: &str, params: &[&str]) -> Result<Vec<T>, Error>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
        for<'q> <DB as Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        for<'q> String: sqlx::Type<DB> + sqlx::Encode<'q, DB>,
    {
        let mut q = sqlx::query_as::<_, T>(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        let result = self.rt.block_on(q.fetch_all(&self.pool))
            .map_err(|e| Error::Generic(format!("Fetch all failed: {}", e)))?;
        Ok(result)
    }

    pub fn fetch_scalar<T>(&self, query: &str, params: &[&str]) -> Result<Option<T>, Error>
    where
        T: Send + Unpin + sqlx::Type<DB> + for<'r> sqlx::Decode<'r, DB>,
        for<'q> <DB as Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        for<'q> String: sqlx::Type<DB> + sqlx::Encode<'q, DB>,
        // Add this constraint:
        usize: sqlx::ColumnIndex<<DB as sqlx::Database>::Row>,
    {
        let mut q = sqlx::query_scalar::<_, T>(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        let result = self.rt.block_on(q.fetch_optional(&self.pool))
            .map_err(|e| Error::Generic(format!("Fetch scalar failed: {}", e)))?;
        Ok(result)
    }

    pub fn fetch_all_as<T>(&self, query: &str, params: &[&str]) -> Result<Vec<T>, Error>
    where
        T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row>,
        for<'q> <DB as Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
        for<'q> String: sqlx::Type<DB> + sqlx::Encode<'q, DB>,
    {
        let mut q = sqlx::query_as::<_, T>(query);
        for param in params {
            q = q.bind(param.to_string());
        }
        self.rt.block_on(q.fetch_all(&self.pool))
            .map_err(|e| Error::Generic(format!("Fetch all unwrapped failed: {}", e)))
    }

}







