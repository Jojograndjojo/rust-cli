mod first_level;
mod second_level;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use first_level::{FirstLevel, FirstLevelTrait};
use mockall::predicate::*;
use second_level::{SecondLevel, SecondLevelTrait};

pub fn run() -> Result<(), anyhow::Error> {
    let first_level = FirstLevel {};
    let second_level = SecondLevel {};
    let cli: Cli = Cli::parse();

    execute_cli_commands(cli, &first_level, &second_level).context("execute cli commands error")
}

fn execute_cli_commands(
    cli: Cli,
    first_level: &impl FirstLevelTrait,
    second_level: &impl SecondLevelTrait,
) -> Result<(), anyhow::Error> {
    match cli.command {
        Commands::FirstLevelSubCommand(first_level_sub_command) => {
            run_first_level_sub_command(first_level_sub_command, first_level, second_level)
                .context("first level sub command error")
        }
    }
}

fn run_first_level_sub_command(
    first_level_sub_command: FirstLevelSubCommand,
    first_level: &impl FirstLevelTrait,
    second_level: &impl SecondLevelTrait,
) -> Result<(), anyhow::Error> {
    match first_level_sub_command.command {
        Some(FirstLevelCommands::SecondLevelSubCommand(second_level_sub_command)) => {
            process_second_level_sub_command(second_level_sub_command, second_level)
        }
        None => process_first_level_command(first_level_sub_command, first_level),
    }
}

fn process_first_level_command(
    first_level_sub_command: FirstLevelSubCommand,
    first_level: &impl FirstLevelTrait,
) -> Result<(), anyhow::Error> {
    if first_level_sub_command.first_level_flag.is_none()
        && first_level_sub_command.command.is_none()
    {
        _ = FirstLevelSubCommand::command()
            .print_help()
            .context("first level sub command help error");
        return Ok(());
    }
    _ = first_level
        .first_level_method(first_level_sub_command.first_level_flag)
        .context("first level sub command error");
    Ok(())
}

fn process_second_level_sub_command(
    second_level_sub_command: SecondLevelSubCommand,
    second_level: &impl SecondLevelTrait,
) -> Result<(), anyhow::Error> {
    _ = run_second_level_sub_command(second_level_sub_command, second_level)
        .context("second level sub command error");
    Ok(())
}

fn run_second_level_sub_command(
    second_level_sub_command: SecondLevelSubCommand,
    second_level: &impl SecondLevelTrait,
) -> Result<(), anyhow::Error> {
    if second_level_sub_command.second_level_flag.is_none() {
        _ = SecondLevelSubCommand::command()
            .print_help()
            .context("second level sub command help error");
        return Ok(());
    }

    _ = second_level
        .second_level_method(second_level_sub_command.second_level_flag)
        .context("second level sub command error");
    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    FirstLevelSubCommand(FirstLevelSubCommand),
}

#[derive(Debug, Subcommand)]
enum FirstLevelCommands {
    SecondLevelSubCommand(SecondLevelSubCommand),
}

#[derive(Debug, Parser)]
struct FirstLevelSubCommand {
    /// First level flag
    #[arg(short, long)]
    first_level_flag: Option<String>,
    #[command(subcommand)]
    command: Option<FirstLevelCommands>,
}

#[derive(Debug, Parser)]
struct SecondLevelSubCommand {
    /// Second level flag
    #[arg(short, long)]
    second_level_flag: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use first_level::MockFirstLevelTrait;
    use second_level::MockSecondLevelTrait;
    use serial_test::serial;
    use std::fs;
    use stdio_override::StdoutOverride;

    const STDOUT_FILE: &str = "test-stdout.txt";

    #[test]
    #[serial]
    fn test_run_second_level_sub_command_print_help_when_second_level_flag_is_none() {
        let guard = StdoutOverride::override_file(STDOUT_FILE).unwrap();
        let second_level_sub_command = SecondLevelSubCommand {
            second_level_flag: None,
        };

        let first_level_sub_command = FirstLevelSubCommand {
            first_level_flag: None,
            command: Some(FirstLevelCommands::SecondLevelSubCommand(
                second_level_sub_command,
            )),
        };

        let cli = Cli {
            command: Commands::FirstLevelSubCommand(first_level_sub_command),
        };

        let mut first_level_mock = MockFirstLevelTrait::new();
        first_level_mock.expect_first_level_method().times(0);

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock.expect_second_level_method().times(0);

        assert!(execute_cli_commands(cli, &first_level_mock, &second_level_mock).is_ok());

        let second_level_expected_help = r#"Usage: rust-cli [OPTIONS]

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
            second_level_flag: Some(flag),
        };

        let first_level_sub_command = FirstLevelSubCommand {
            first_level_flag: None,
            command: Some(FirstLevelCommands::SecondLevelSubCommand(
                second_level_sub_command,
            )),
        };

        let cli = Cli {
            command: Commands::FirstLevelSubCommand(first_level_sub_command),
        };

        let mut first_level_mock = MockFirstLevelTrait::new();
        first_level_mock.expect_first_level_method().times(0);

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock
            .expect_second_level_method()
            .with(eq(Some("flag".to_string())))
            .times(1)
            .returning(|_| Ok(()));

        assert!(execute_cli_commands(cli, &first_level_mock, &second_level_mock).is_ok());
    }

    #[test]
    #[serial]
    fn test_run_first_level_command_print_help_when_first_level_sub_command_flag_and_command_are_none(
    ) {
        let guard = StdoutOverride::override_file(&STDOUT_FILE).unwrap();

        let first_level_sub_command = FirstLevelSubCommand {
            first_level_flag: None,
            command: None,
        };

        let cli = Cli {
            command: Commands::FirstLevelSubCommand(first_level_sub_command),
        };

        let mut first_level_mock = MockFirstLevelTrait::new();
        first_level_mock.expect_first_level_method().times(0);

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock.expect_second_level_method().times(0);

        assert!(execute_cli_commands(cli, &first_level_mock, &second_level_mock).is_ok());

        let first_level_expected_help = "Usage: rust-cli [OPTIONS] [COMMAND]

Commands:
  second-level-sub-command\u{20}\u{20}
  help                      Print this message or the help of the given subcommand(s)

Options:
  -f, --first-level-flag <FIRST_LEVEL_FLAG>  First level flag
  -h, --help                                 Print help
";

        let first_level_actual_help = fs::read_to_string(&STDOUT_FILE).unwrap();

        assert!(first_level_actual_help.contains(first_level_expected_help));
        drop(guard);

        fs::remove_file(&STDOUT_FILE).unwrap();
    }

    #[test]
    fn test_run_first_level_command() {
        let flag = String::from("flag");
        let first_level_sub_command = FirstLevelSubCommand {
            first_level_flag: Some(flag),
            command: None,
        };

        let cli = Cli {
            command: Commands::FirstLevelSubCommand(first_level_sub_command),
        };

        let mut first_level_mock = MockFirstLevelTrait::new();
        first_level_mock
            .expect_first_level_method()
            .with(eq(Some("flag".to_string())))
            .times(1)
            .returning(|_| Ok(()));

        let mut second_level_mock = MockSecondLevelTrait::new();
        second_level_mock.expect_second_level_method().times(0);

        assert!(execute_cli_commands(cli, &first_level_mock, &second_level_mock).is_ok());
    }
}
