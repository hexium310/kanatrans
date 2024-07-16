use clap::Parser;
#[cfg(feature = "cli")]
use cli::options::Options;

/// Print Katakana or ARPAbet converted from word composed of English alphabet
#[derive(Parser)]
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
