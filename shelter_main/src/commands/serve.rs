use std::sync::Arc;
use clap::{Arg, ArgMatches, Command, value_parser};
use tower_http::trace::TraceLayer;

use crate::settings::Settings;
use crate::state::ApplicationState;

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("指定TCP监听端口")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        start_tokio(port, _settings)?;
    }

    Ok(())
}


fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let state = Arc::new(ApplicationState::new(settings)?);
            let routes = crate::api::configure(state).layer(TraceLayer::new_for_http());
            tracing::info!("starting axum on port {}", port);
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
            axum::serve(listener, routes.into_make_service()).await?;
            Ok::<(), anyhow::Error>(())
        })?;
    std::process::exit(0);
}