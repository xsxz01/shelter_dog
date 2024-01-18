mod hello;
mod serve;
mod migrate;
mod createadmin;

use clap::{ArgMatches, Command};
use crate::settings::Settings;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(createadmin::configure())
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, _settings)?;
    serve::handle(matches, _settings)?;
    migrate::handle(matches, _settings)?;
    createadmin::handle(matches, _settings)?;

    Ok(())
}