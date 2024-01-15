use clap::{Command, Arg};
use dotenv::dotenv;
use shelter_main::commands;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let mut command = Command::new("微服务命令行")
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
    let _matches = command.get_matches();
    commands::handle(&_matches)?;
    Ok(())
}
