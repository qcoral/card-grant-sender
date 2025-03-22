use std::{error::Error, io, process};

#[derive(serde::Deserialize)]
struct Row<'a> {
    slack_id: &'a str,
    email: &'a str,
    bom: &'a str,
}


fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let row: Row = record.deserialize(None)?;
        println!("{}", row.slack_id);
        println!("{}", row.email);
        println!("{}", row.bom);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}