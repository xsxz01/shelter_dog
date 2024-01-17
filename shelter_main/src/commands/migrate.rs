use clap::{ArgMatches, Command};
use sea_orm::Database;
use migration::{Migrator, MigratorTrait};
use crate::settings::{Settings};

pub fn configure() -> Command {
    Command::new("migrate").about("进行数据库迁移")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("migrate") {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().unwrap_or("".to_string());
                let conn = Database::connect(db_url)
                    .await
                    .expect("数据库连接失败");
                Migrator::up(&conn, None).await.unwrap();
            })
    }
        Ok(())
}