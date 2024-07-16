use clap::Parser;
#[cfg(feature = "cli")]
use cli::options::Options;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub(crate) struct Command {
    #[arg(short, long, conflicts_with_all = ["arpabet", "katakana", "words"])]
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
        cfg!(feature = "server") && self.serve
    }
}
