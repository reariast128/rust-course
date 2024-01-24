use csv::{ ReaderBuilder, StringRecord };
use std::{ fs };

const FILENAME: &str = "history.csv";

fn read_game(mut game_content: csv::Reader<&[u8]>) {
    for record in game_content.records() {
        println!("Texto: {}", record.unwrap().get(2).unwrap().trim());
    }
}

fn main() {
    let raw_content: Result<String, std::io::Error> = fs::read_to_string(FILENAME);
    let reader_builder: csv::Reader<&[u8]>;
    match raw_content {
        Ok(file_reader) => {
            reader_builder = ReaderBuilder::new()
                .delimiter(b';')
                .from_reader(file_reader.as_bytes());
            read_game(reader_builder);
        }
        Err(error) => {
            println!("Ha ocurrido un error: {}", error);
        }
    }
}
