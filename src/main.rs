use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Insert {
        #[arg(long)]
        player: String,
        #[arg(long)]
        position: String,
        #[arg(long)]
        id: String,
        #[arg(long = "draft-year")]
        draft_year: i32,
        #[arg(long = "projected-spm")]
        projected_spm: f64,
        #[arg(long)]
        superstar: f64,
        #[arg(long)]
        starter: f64,
        #[arg(long = "role-player")]
        role_player: f64,
        #[arg(long)]
        bust: f64,
    },
    Read {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long = "new-player")]
        new_player: String,
        #[arg(long = "new-position")]
        new_position: String,
        #[arg(long = "new-draft-year")]
        new_draft_year: i32,
        #[arg(long = "new-projected-spm")]
        new_projected_spm: f64,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Establish database connection
    let conn = Connection::open("data/nba_db.sqlite")?;

    match args.command {
        Commands::Insert {
            player,
            position,
            id,
            draft_year,
            projected_spm,
            superstar,
            starter,
            role_player,
            bust,
        } => {
            rust_ind8::insert_record(&conn, &player, &position, &id, draft_year, projected_spm, superstar, starter, role_player, bust)?;
            println!("Inserted record with ID: {}", id);
        }
        Commands::Read { id } => {
            match rust_ind8::read_record_by_id(&conn, &id)? {
                Some(record) => {
                    println!("Record found: {:?}", record);
                }
                None => {
                    println!("No record found with ID: {}", id);
                }
            }
        }
        Commands::Update {
            id,
            new_player,
            new_position,
            new_draft_year,
            new_projected_spm,
        } => {
            rust_ind8::update_record(&conn, &id, &new_player, &new_position, new_draft_year, new_projected_spm)?;
            println!("Updated record with ID: {}", id);
        }
        Commands::Delete { id } => {
            rust_ind8::delete_record(&conn, &id)?;
            println!("Deleted record with ID: {}", id);
        }
    }

    Ok(())
}
