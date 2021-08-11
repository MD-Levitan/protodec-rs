pub mod args;
pub mod parser;
pub mod proto;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

use args::get_config;
use parser::parser::{FullParser, Parser, SimpleParser};

use std::fs::File;
use std::io::Read;

fn init_log(
    logfile: &str,
    debug_level: LevelFilter,
) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let stdout = ConsoleAppender::builder().build();

    // let requests = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
    //     .build(logfile)
    //     .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        //.appender(Appender::builder().build("requests", Box::new(requests)))
        //.logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        // .logger(Logger::builder()
        //     .appender("requests")
        //     .additive(false)
        //     .build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(debug_level))?;

    let handle = log4rs::init_config(config)?;

    Ok(())
}

fn main() {
    let config = get_config();
    let mut data = Vec::new();

    init_log("", config.verbose_level).unwrap();

    let mut f = File::open(config.file).expect("Something went wrong reading the file");
    f.read_to_end(&mut data).expect("Failed to read data");

    let parser = FullParser::new();
    let message = parser.deserialize(&data).unwrap();
    for field in message.fields.iter() {
        // let b: &StringField = match field.as_any().downcast_ref::<StringField>() {
        //     Some(b) => b,
        //     None => panic!("&a isn't a B!"),
        // };
        println!("{}", field.repr());
    }
}
