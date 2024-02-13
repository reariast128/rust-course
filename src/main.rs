use csv::ReaderBuilder;
use std::fs;

const FILENAME: &str = "history.csv";

fn read_game(mut game_content: csv::Reader<&[u8]>) {
    /* This function prints the CSV content. */
    for record in game_content.records() {
        println!("Texto: {}", record.unwrap().get(2).unwrap().trim());
    }
}

fn main() {
    // Reading the CSV in raw.
    let raw_content: Result<String, std::io::Error> = fs::read_to_string(FILENAME);
    let reader_builder: csv::Reader<&[u8]>;
    
    match raw_content {
        Ok(file_reader) => {
            // If Ok, this block creates a Reader, having that the CSV separator is ';'. Then, read_game() is executed.
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
