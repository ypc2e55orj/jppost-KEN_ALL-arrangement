use mysql;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    host: String,
    port: usize,
    user: String,
    password: String,
    database_name: String,
    table_name: String,
    multiple_insert: usize,
}

pub struct Database {
    pool: mysql::Pool,
    database_config: DatabaseConfig,
}

impl Database {
    pub fn new(database_config: DatabaseConfig) -> Database {
        let pool = mysql::Pool::new(format!(
            "mysql://{}:{}@{}:{}/{}",
            database_config.user,
            database_config.password,
            database_config.host,
            database_config.port,
            database_config.database_name
        ))
        .expect("Failed to create pools!");

        Database {
            pool,
            database_config,
        }
    }

    fn query(&self, query: String) -> Result<Vec<mysql::Row>, mysql::Error> {
        let result = self.pool.prep_exec(&query, ())?;
        let mut result_vec: Vec<mysql::Row> = Vec::new();

        for wrapped_row in result {
            if let Ok(row) = wrapped_row {
                result_vec.push(row);
            }
        }

        Ok(result_vec)
    }
}
