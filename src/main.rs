mod second_level;

use clap::{CommandFactory, Parser, Subcommand};
use mockall::predicate::*;
use second_level::{SecondLevel, SecondLevelTrait};

fn main() {
    let second_level = SecondLevel {};
    let cli: Cli = Cli::parse();
    _ = run_cli(cli, &second_level);
}

fn run_cli(
    cli: Cli,
    second_level: &impl SecondLevelTrait,
) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::SecondLevelSubCommand(second_level_sub_command) => {
            match run_second_level_sub_command(second_level_sub_command, second_level) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("second level sub command error: {}", err))?,
            }
        }
    }
}

fn run_second_level_sub_command(
    second_level_sub_command: SecondLevelSubCommand,
    second_level: &impl SecondLevelTrait,
) -> Result<(), Box<dyn std::error::Error>> {
    if second_level_sub_command.second_level_flag.is_empty() {
        match SecondLevelSubCommand::command().print_help() {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                Err(format!("second level sub command help error: {}", err))?;
            }
        }
    }

    match second_level.second_level_method(second_level_sub_command.second_level_flag) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("second level sub command error: {}", err))?,
    }
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
mod tests {
    use super::*;
    use second_level::MockSecondLevelTrait;
    use std::fs;
    use stdio_override::StdoutOverride;

    const STDOUT_FILE: &str = "test-stdout.txt";

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

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock.expect_second_level_method().times(0);

        assert!(run_cli(cli, &second_level_mock).is_ok());

        let second_level_expected_help = r#"Usage: rust-cli --second-level-flag <SECOND_LEVEL_FLAG>

Options:
  -s, --second-level-flag <SECOND_LEVEL_FLAG>  Second level flag
  -h, --help                                   Print help
"#;

        let second_level_actual_help = fs::read_to_string(STDOUT_FILE).unwrap();

        assert!(second_level_actual_help.contains(second_level_expected_help));

        drop(guard);

        fs::remove_file(STDOUT_FILE).unwrap();
    }

    #[test]
    fn test_run_second_level_sub_command() {
        let flag = String::from("flag");
        let second_level_sub_command = SecondLevelSubCommand {
            second_level_flag: flag,
        };

        let cli = Cli {
            command: Commands::SecondLevelSubCommand(second_level_sub_command),
        };

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock
            .expect_second_level_method()
            .with(eq("flag".to_string()))
            .times(1)
            .returning(|_| Ok(()));

        assert!(run_cli(cli, &second_level_mock).is_ok());
    }
}
