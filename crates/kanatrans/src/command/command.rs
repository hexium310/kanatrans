use clap::Parser;
#[cfg(feature = "cli")]
use cli::options::Options;

/// Print Katakana or ARPAbet converted from word composed of English alphabet
#[derive(Debug, Parser, PartialEq)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub(crate) struct Command {
    #[cfg(feature = "server")]
    /// Start Kanatrans server
    #[arg(short, long, exclusive(true))]
    pub(crate) serve: bool,

    #[cfg(feature = "cli")]
    #[command(flatten)]
    pub(crate) cli: Options,
}

#[derive(Debug)]
pub(crate) enum CommandKind {
    Serve,
    Cli,
}

impl Command {
    pub(crate) fn kind(&self) -> CommandKind {
        if self.is_serve() {
            return CommandKind::Serve;
        }

        CommandKind::Cli
    }

    fn is_serve(&self) -> bool {
        #[cfg(feature = "server")]
        return self.serve;

        #[cfg(not(feature = "server"))]
        false
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use cli::options::{Options, OutputKindArgs};

    use crate::command::Command;

    #[test]
    fn no_args() {
        let command = Command::try_parse_from(["kanatrans"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Err(clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand)
        );
    }

    #[test]
    fn cli_with_no_flags() {
        let command = Command::try_parse_from(["kanatrans", "threshold"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Ok(Command {
                serve: false,
                cli: Options {
                    words: ["threshold"].map(Into::into).to_vec(),
                    output_kind: OutputKindArgs {
                        arpabet: false,
                        katakana: false,
                    },
                },
            })
        );
    }

    #[test]
    fn cli_with_katakana() {
        let command = Command::try_parse_from(["kanatrans", "--katakana", "threshold"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Ok(Command {
                serve: false,
                cli: Options {
                    words: ["threshold"].map(Into::into).to_vec(),
                    output_kind: OutputKindArgs {
                        arpabet: false,
                        katakana: true,
                    },
                },
            })
        );
    }

    #[test]
    fn cli_with_arpabet() {
        let command = Command::try_parse_from(["kanatrans", "--arpabet", "threshold"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Ok(Command {
                serve: false,
                cli: Options {
                    words: ["threshold"].map(Into::into).to_vec(),
                    output_kind: OutputKindArgs {
                        arpabet: true,
                        katakana: false,
                    },
                },
            })
        );
    }

    #[test]
    fn cli_katakana_and_arapabet() {
        let command = Command::try_parse_from(["kanatrans", "--katakana", "--arpabet", "threshold"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Ok(Command {
                serve: false,
                cli: Options {
                    words: ["threshold"].map(Into::into).to_vec(),
                    output_kind: OutputKindArgs {
                        arpabet: true,
                        katakana: true,
                    },
                },
            })
        );
    }

    #[test]
    fn serve() {
        let command = Command::try_parse_from(["kanatrans", "--serve"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Ok(Command {
                serve: true,
                cli: Options {
                    words: vec![],
                    output_kind: OutputKindArgs {
                        arpabet: false,
                        katakana: false,
                    },
                },
            })
        );
    }

    #[test]
    fn serve_should_conflict_cli() {
        let command = Command::try_parse_from(["kanatrans", "--serve", "--katakana", "--arpabet", "threshold"]);

        assert_eq!(
            command.map_err(|e| e.kind()),
            Err(clap::error::ErrorKind::ArgumentConflict)
        );
    }
}
