use std::io::Write;

use clap::Parser;
use env_logger::Env;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(short, long, default_value = "info")]
    pub level: String,

    #[clap(
        short,
        long,
        default_value = "0.0.0.0:8080",
        env = "MICRO_SERVER_ADDRESS"
    )]
    pub addr: String,
}

pub fn parse() -> Args {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or(&args.level))
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}] {}",
                ts,
                record.level(),
                record.module_path().unwrap(),
                record.args()
            )
        })
        .init();

    args
}
