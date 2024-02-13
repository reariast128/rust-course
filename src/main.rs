use csv::{ ReaderBuilder, StringRecord };
use std::collections::hash_map;
use std::fs;

const FILENAME: &str = "history.csv";

fn read_game(mut game_content: csv::Reader<&[u8]>) {
    /* This function prints the CSV content. */
    /*for record in game_content.records() {
        println!("Texto: {}", record.unwrap().get(2).unwrap().trim());
    }*/

    let mut facts: Vec<HistoryFact> = vec![];
    for record in game_content.records() {
        let record = record.unwrap();
        let fact = HistoryFact::new(record);

        println!("{:?}", fact);
        facts.push(fact);
    }
}

#[derive(Debug)]
struct HistoryFact {
    fact_type: String,
    tag: String,
    text: String,
    life: i32,
}

impl HistoryFact {
    fn new(content: StringRecord) -> HistoryFact {
        return HistoryFact {
            fact_type: content.get(0).unwrap().to_string(),
            tag: content.get(1).unwrap().to_string(),
            text: content.get(2).unwrap().to_string(),
            life: content.get(3).unwrap().trim().parse().unwrap_or(0),
        };
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
