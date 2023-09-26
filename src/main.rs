use clap::Parser;

fn main() {
    // Entry point for the tool interface

    let cli: Cli = Cli::parse();

    // Upack the optional off-hand options
    let offhand_attacks: i32 = unload_offhand_attacks(cli.offhand_attacks);
    let offhand_weapon: String = unload_offhand_weapon(cli.offhand_weapon);

    // Process the information and capture results as a polars DataFrame
    let output_df = match dpr_simulator::process_simulation(
        cli.ac_targets,
        cli.to_hit,
        cli.mainhand_attacks,
        cli.mainhand_weapon,
        offhand_attacks,
        offhand_weapon,
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

fn unload_offhand_attacks(offhand_option: Option<i32>) -> i32 {
    match offhand_option {
        Some(o) => o,
        None => 0,
    }
}

fn unload_offhand_weapon(offhand_option: Option<String>) -> String {
    match offhand_option {
        Some(s) => s,
        None => "".to_string(),
    }
}

fn report_error(error_msg: &str) {
    println!("ERROR: {}", error_msg);
}

#[derive(Parser)]
struct Cli {
    // Build the CLI input arguments and options.
    /// To-Hit modifier
    #[arg(short = 't', long, value_name = "TO HIT")]
    to_hit: i32,

    /// Path to save results (Apache parquet format)
    #[arg(short, long, value_name = "OUTPUT FILE")]
    output: String,

    /// Number of main-hand attackes to make per turn
    #[arg(short = 'm', long, value_name = "MAINHAND ATTACKS")]
    mainhand_attacks: i32,

    /// Details of the mainhand weapon (e.g. 1d8+5)
    #[arg(short = 'w', long, value_name = "MAINHAND WEAPON")]
    mainhand_weapon: String,

    /// Number of off-hand attackes to make per turn (optional)
    #[arg(long, value_name = "OFFHAND ATTACKS")]
    offhand_attacks: Option<i32>,

    /// Details of the offhand weapon (optional)
    #[arg(long, value_name = "OFFHAND WEAPON")]
    offhand_weapon: Option<String>,

    /// Number of turns to simulate (default 1,000,000)
    #[arg(short, long, value_name = "NUMBER TURNS", default_value_t = 1_000_000)]
    number_turns: i32,

    /// Space-delimited AC values to test against (default 12, 14, 16, 18, 20)
    #[arg(short, long, value_name = "AC TARGETS", num_args = 1.., value_delimiter = ' ', default_values_t = vec![12, 14, 16, 18, 20])]
    ac_targets: Vec<i32>,
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unload_offhand_attacks() {
        // Test the unload_offhand_attacks() function when a value is provided.

        let exp_value = 3;
        let obs_value = unload_offhand_attacks(Some(3));

        assert_eq!(exp_value, obs_value);
    }

    #[test]
    fn test_unload_offhand_attacks_none() {
        // Test the unload_offhand_attacks() function when nothing is provided.

        let obs_value = unload_offhand_attacks(None);
        assert_eq!(0, obs_value);
    }

    #[test]
    fn test_unload_offhand_weapon() {
        // Test the unload_offhand_weapon() function when a value is provided.

        let exp_value = "a";
        let obs_value = unload_offhand_weapon(Some("a".to_string()));

        assert_eq!(exp_value, &obs_value);
    }

    #[test]
    fn test_unload_offhand_weapon_none() {
        // Test the unload_offhand_weapon() function when nothing is provided.

        let obs_value = unload_offhand_weapon(None);
        assert_eq!("", &obs_value);
    }
}

//endregion
