use clap::{crate_version, App, AppSettings, Arg};
use log::LevelFilter;
use core::str::FromStr;

pub struct Config {
    pub file: String,
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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("verbose_level")
                .short("V")
                .long("verobose_level")
                .help("Verbose level")
                .default_value("INFO"),
        );
    let args = app.clone().get_matches();

    let file = match args.value_of("file") {
        Some(val) => val,
        None => "",
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
        file: file.to_string(),
        verbose_level: verbose,
    }
}
