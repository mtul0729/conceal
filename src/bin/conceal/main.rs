mod cli;

use clap::Parser;

use cli::Cli;

#[cfg(any(freedesktop, target_os = "windows"))]
fn main() {
    use std::process;

    use conceal::handler;

    use cli::SubCommand;

    let cli = Cli::parse();

    use SubCommand::*;
    let result = match cli.command {
        List { all, since, before } => handler::list(all, since.as_deref(), before.as_deref()),
        Restore { finder } => handler::restore(finder.cmd()),
        Delete { finder, all } => handler::delete(finder.cmd(), all),
        Clean { all, since, before } => handler::clean(all, since.as_deref(), before.as_deref()),
    };

    if let Err(e) = result {
        e.print("conceal");
        process::exit(1);
    }
}

#[cfg(target_os = "macos")]
fn main() {
    Cli::parse();
}
