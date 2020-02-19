use crate::ken_all::ken_all::KenAll;
use mysql;

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

    /*
        CREATE TABLE ken_all
        (
            id INT(11) NOT NULL AUTO_INCREMENT,
            postal_code VARCHAR(255) NOT NULL,
            prefecture_katakana_half VARCHAR(255) NOT NULL,
            city_katakana_half VARCHAR(255) NOT NULL,
            town_katakana_half VARCHAR(255) NOT NULL,
            prefecture_kanji VARCHAR(255) NOT NULL,
            city_kanji VARCHAR(255) NOT NULL,
            town_kanji VARCHAR(255) NOT NULL,
            PRIMARY KEY(id)
        )
        DEFAULT CHARSET=utf8mb4;
    */
    pub fn multiple_insert_ken_all(&self, ken_all_vec: Vec<KenAll>) {
        let ken_all_vec_len = ken_all_vec.len();
        let columns = String::from("postal_code, prefecture_katakana_half, city_katakana_half, town_katakana_half, prefecture_kanji, city_kanji, town_kanji");

        let mut values = String::from("");
        let mut current_multiples: usize = 1;

        for val in ken_all_vec {
            values += &format!(
                "('{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                val.postal_code,
                val.prefecture_katakana_half,
                val.city_katakana_half,
                val.town_katakana_half,
                val.prefecture_kanji,
                val.city_kanji,
                val.town_kanji
            );

            if current_multiples == self.database_config.multiple_insert
                || current_multiples == ken_all_vec_len
            {
                self.query(format!(
                    "INSERT INTO {}({}) VALUES {}",
                    self.database_config.table_name, columns, values
                ))
                .expect("Failed to insert a value!");

                values = String::from("");
                current_multiples = 1;
            } else {
                values = values + ", ";
                current_multiples += 1;
            }
        }
    }
}
