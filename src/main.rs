use clap::{CommandFactory, Parser, Subcommand};

fn main() {
    let cli: Cli = Cli::parse();
    _ = run_cli(cli);
}

fn run_cli(
    cli: Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::SecondLevelSubCommand(second_level_sub_command) => {
            match run_second_level_sub_command(second_level_sub_command) {
                Ok(_) => {}
                Err(err) => Err(format!("second level sub command error: {}", err)).unwrap(),
            }
        }
    }
    return Ok(());
}

fn run_second_level_sub_command(
    second_level_sub_command: SecondLevelSubCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    if second_level_sub_command.second_level_flag.is_empty() {
        match SecondLevelSubCommand::command().print_help() {
            Ok(_) => {}
            Err(err) => Err(format!("second level sub command help error: {}", err)).unwrap(),
        }
    }
    return Ok(());
}

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    SecondLevelSubCommand(SecondLevelSubCommand),
}

#[derive(Debug, Parser)]
struct SecondLevelSubCommand {
    /// Second level flag
    #[arg(short, long)]
    second_level_flag: String,
}

#[cfg(test)]
use test_env_helpers::*;

#[after_each]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use stdio_override::StdoutOverride;

    const STDOUT_FILE: &str = "test-stdout.txt";
    fn after_each() {
        fs::remove_file(STDOUT_FILE).unwrap();
    }

    #[test]
    fn test_run_second_level_sub_command_print_help_when_second_level_flag_is_empty() {
        let flag = String::from("");
        let guard = StdoutOverride::override_file(STDOUT_FILE).unwrap();
        let second_level_sub_command = SecondLevelSubCommand {
            second_level_flag: flag,
        };

        let cli = Cli {
            command: Commands::SecondLevelSubCommand(second_level_sub_command),
        };

        _ = run_cli(cli);

        let second_level_expected_help = r#"Usage: rust-cli --second-level-flag <SECOND_LEVEL_FLAG>

Options:
  -s, --second-level-flag <SECOND_LEVEL_FLAG>  Second level flag
  -h, --help                                   Print help
"#;

        let second_level_actual_help = fs::read_to_string(STDOUT_FILE).unwrap();

        assert_eq!(second_level_expected_help, second_level_actual_help);

        drop(guard);
    }
}
