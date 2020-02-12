use csv;
use encoding::{all::WINDOWS_31J, DecoderTrap, Encoding};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize, Clone)]
pub struct KenAll {
    pub JIS_X_0401_0402: usize,
    pub old_postal_code: String,
    pub postal_code: String,
    pub prefecture_katakana_half: String,
    pub city_katakana_half: String,
    pub town_katakana_half: String,
    pub prefecture_kanji: String,
    pub city_kanji: String,
    pub town_kanji: String,
    pub flag_multiple_postal_code: u8,
    pub flag_koaza: u8,
    pub flag_chome: u8,
    pub flag_multiple_town: u8,
    pub flag_update: u8,
    pub flag_update_reason: u8,
}

impl KenAll {
    fn read_buffer(path: &str) -> Result<Vec<u8>, std::io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer: Vec<u8> = vec![];

        reader.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn sjis_buffer_to_utf8_string(buffer: Vec<u8>) -> Result<String, std::borrow::Cow<'static, str>>{
        let mut utf8_content = String::from("");

        WINDOWS_31J.decode_to(&buffer, DecoderTrap::Replace, &mut utf8_content)?;

        Ok(utf8_content)
    }

    fn string_to_vec(string: String) -> Result<Vec<KenAll>, csv::Error> {
        let mut reader = csv::Reader::from_reader(string.as_bytes());
        let mut ken_all_vec: Vec<KenAll> = vec![];

        for result in reader.records() {
            let record: KenAll = result?.deserialize(None)?;
            ken_all_vec.push(record.clone());
        }

        Ok(ken_all_vec)
    }

    pub fn read(path: &str) -> Vec<KenAll> {
        let buffer = KenAll::read_buffer(path).expect("Failed to read a file!");
        let utf8_content = KenAll::sjis_buffer_to_utf8_string(buffer).expect("Failed to convert sjis to utf-8!");

        KenAll::string_to_vec(utf8_content).expect("Failed to convert csv to struct!")
    }
}
