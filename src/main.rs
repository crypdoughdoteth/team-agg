use std::fs::OpenOptions;
use clap::{Parser, Subcommand};
use polars::prelude::*;

#[derive(Parser, Debug)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    New {
        #[clap(short)]
        team_name: String,
        #[clap(short)]
        bounties: Vec<String>,
        #[clap(short)]
        note: Option<String>,
    },
    // Note {
    //     team_name: String,
    //     note: String,
    // },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load csv

    let mut csv: DataFrame = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("./hack.csv".into()))?
        .finish()
        .unwrap_or_default();

    let args = App::parse();

    match args.command {
        Command::New {
            team_name,
            bounties,
            note,
        } => new(&mut csv, team_name, bounties, note)?,
        //     Command::Update{ team_name, note } => todo!(),
    };

    Ok(())
}

pub fn new(
    df: &mut DataFrame,
    name: String,
    bounties: Vec<String>,
    note: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = Column::new("name".into(), [name]);
    let bounties = Column::new("bounties".into(), bounties);
    let note = Column::new("note".into(), [note]);
    let new_df = DataFrame::new(vec![name, bounties, note])?;
    let mut df_new = df.vstack(&new_df)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("hack.csv")?;

    CsvWriter::new(&mut file).finish(&mut df_new).unwrap();

    Ok(())
}
