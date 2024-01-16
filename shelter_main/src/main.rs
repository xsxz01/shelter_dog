use clap::{Command, Arg};
use dotenv::dotenv;
use shelter_main::commands;
use shelter_main::settings;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let mut command  = Command::new("微服务命令行")
        .version("ver 1.0")
        .author("Pang<pangyuyu@illusiontech.cn>")
        .about("Rust微服务程序命令行程序。")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("配置文件路径")
                .default_value("config.json"),
        );
    command = commands::configure(command);
    let matches = command.get_matches();
    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "SHELTER")?;
    commands::handle(&matches, &settings)?;
    Ok(())
}
