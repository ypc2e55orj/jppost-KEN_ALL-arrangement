use crate::ken_all::database::Database;
use crate::ken_all::config::Config;

#[derive(Debug, Clone)]
pub struct KenAll {
    zip_code: String,
    prefecture_katakana_half: String,
    city_katakana_half: String,
    town_katakana_half: String,
    prefecture_kanji: String,
    city_kanji: String,
    town_kanji: String,
}
