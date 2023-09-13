use clap::{crate_version, App, AppSettings, Arg};
use core::str::FromStr;
use hex::decode;
use log::LevelFilter;
pub struct Config {
    pub file: Option<String>,
    pub data: Option<Vec<u8>>,
    pub verbose_level: LevelFilter,
}

pub fn get_config() -> Config {
    let app = App::new("protodec")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author("kusok <ovsyanka@protonmail.com>")
        .about("Protobuf reverse tool")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("File to decode")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose_level")
                .short("V")
                .long("verbose_level")
                .help("Verbose level")
                .default_value("INFO"),
        )
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .help("Data in hex to decode")
                .takes_value(true),
        );
    let args = app.clone().get_matches();

    let file = match args.value_of("file") {
        Some(val) => Some(val.to_string()),
        None => None,
    };

    let data = match args.value_of("data") {
        Some(val) => Some(match decode(val.to_string()) {
            Err(_) => {
                println!("Unable to parse 'data' value");
                std::process::exit(1);
            }
            Ok(v) => v,
        }),
        None => None,
    };

    let verbose = match args.value_of("verbose_level") {
        Some(val) => match LevelFilter::from_str(val) {
            Ok(v) => v,
            Err(_) => {
                //TODO: add norm error
                println!("Unable to parse 'verbose' value");
                std::process::exit(1);
            }
        },
        None => LevelFilter::Info,
    };

    Config {
        file: file,
        data: data,
        verbose_level: verbose,
    }
}
