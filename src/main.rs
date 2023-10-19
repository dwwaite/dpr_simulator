use clap::Parser;
use dpr_simulator::Ruleset;

fn main() {
    // Entry point for the tool interface

    let cli: Cli = Cli::parse();

    // Upack the optional parameters
    let ruleset = match cli.use_pf2e_criticals {
        true => Ruleset::PF2e,
        false => Ruleset::DND5e,
    };

    // MAP...

    // Process the information and capture results as a polars DataFrame
    let output_df = match dpr_simulator::process_simulation(
        cli.ac_targets,
        cli.to_hit,
        cli.weapon_details,
        ruleset,
        cli.number_turns,
    ) {
        Ok(df) => df,
        Err(e) => {
            report_error(&e.to_string());
            std::process::exit(1);
        }
    };

    // Store the output
    match dpr_simulator::write_to_parquet(&cli.output, output_df) {
        Ok(_) => println!("Completed! Results written to file '{}'!", cli.output),
        Err(e) => {
            report_error(&e.to_string());
            std::process::exit(1);
        }
    };
}

fn report_error(error_msg: &str) {
    println!("ERROR: {}", error_msg);
}

#[derive(Parser)]
struct Cli {
    // Build the CLI input arguments and options.
    /// To-Hit modifier, one or one per attack to be made
    #[arg(short = 't', long, value_name = "TO HIT", num_args = 1.., value_delimiter = ' ')]
    to_hit: Vec<i32>,

    /// Space-delimited AC values to test against
    #[arg(short, long, value_name = "AC TARGETS", num_args = 1.., value_delimiter = ' ', default_values_t = vec![12, 14, 16, 18, 20])]
    ac_targets: Vec<i32>,

    /// Details of each attack to be made in the form 1d8+5
    #[arg(short = 'w', long, value_name = "WEAPON DETAILS", num_args = 1.., value_delimiter = ' ')]
    weapon_details: Vec<String>,

    /// Path to save results (Apache parquet format)
    #[arg(short, long, value_name = "OUTPUT FILE")]
    output: String,

    /// Number of turns to simulate
    #[arg(short, long, value_name = "NUMBER TURNS", default_value_t = 1_000_000)]
    number_turns: i32,

    /// Use Pathfinder 2e rules for critical hits and damage calculation
    #[arg(long, default_value_t = false)]
    use_pf2e_criticals: bool,

    /// Apply a Pathfinder 2e MAP progression to the attacks (optional, NOT YET IMPLEMENTED)
    #[arg(long, value_name = "MAP SEQUENCE", num_args = 1.., value_delimiter = ' ')]
    map_sequence: Option<Vec<i32>>,
}
