use clap::Parser;
use dpr_simulator::Ruleset;
use polars::frame::DataFrame;

fn main() {
    // Entry point for the tool interface

    let cli: Cli = Cli::parse();

    // Upack the optional parameters
    let ruleset = match cli.use_pf2e_criticals {
        true => Ruleset::PF2e,
        false => Ruleset::DND5e,
    };

    // Confirm that the hit and attack vectors are equal in length
    let mut hit_vector = cli.to_hit;
    let mut dmg_vector = cli.weapon_details;
    dpr_simulator::equalise_input_vectors(&mut hit_vector, &mut dmg_vector);

    // Process the information and capture results as a polars DataFrame
    let mut output_df = dpr_simulator::process_simulation(
        cli.ac_targets,
        hit_vector,
        dmg_vector,
        ruleset,
        cli.number_turns,
        cli.n_threads,
    );

    // Store the output if required
    if let Some(output_path) = cli.output {
        store_output(&output_path, &mut output_df);
    }

    /* Report the summary results to the user. Currently it appears that the display
        can only be customised using env variables:
            POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES (hide data types)
            POLARS_FMT_TABLE_HIDE_COLUMN_SEPARATOR (hide separator)
    */
    let summary_df = dpr_simulator::summarise_results(output_df);
    println!("{}", summary_df);
}

fn store_output(output_path: &str, output_df: &mut DataFrame) {
    match dpr_simulator::write_to_parquet(output_path, output_df) {
        Ok(_) => println!("Completed! Results written to file '{}'!", output_path),
        Err(e) => {
            println!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Parser)]
struct Cli {
    // Build the CLI input arguments and options.
    /// Space-delimited AC values to test against
    #[arg(short, long, value_name = "AC TARGETS", num_args = 1.., value_delimiter = ' ', default_values_t = vec![12, 14, 16, 18, 20])]
    ac_targets: Vec<i32>,

    /// Details of the attack roll in the form 1d20+X
    #[arg(short = 't', long, value_name = "TO HIT", num_args = 1.., value_delimiter = ' ')]
    to_hit: Vec<String>,

    /// Details of each attack to be made in the form 1dX+Y or 1dX,1dY+Z
    #[arg(short = 'w', long, value_name = "WEAPON DETAILS", num_args = 1.., value_delimiter = ' ')]
    weapon_details: Vec<String>,

    /// Path to save results in Apache parquet format (optional)
    #[arg(short, long, value_name = "OUTPUT FILE")]
    output: Option<String>,

    /// Number of turns to simulate
    #[arg(short, long, value_name = "NUMBER TURNS", default_value_t = 1_000_000)]
    number_turns: i32,

    /// Number of threads for running in multi-threaded mode (optional)
    #[arg(long, value_name = "N THREADS")]
    n_threads: Option<usize>,

    /// Use Pathfinder 2e rules for critical hits and damage calculation
    #[arg(long, default_value_t = false)]
    use_pf2e_criticals: bool,
}
