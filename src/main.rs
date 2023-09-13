pub mod args;
pub mod parser;
pub mod proto;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

use args::get_config;
use parser::parser::{FullParser, Parser, PartialParser};

use std::fs::File;
use std::io::Read;

fn init_log(
    // logfile: &str,
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

    let _handle = log4rs::init_config(config)?;

    Ok(())
}

// fn test() {
//     let buffer = [
//         0x76, 0x2e, 0x30, 0x3a, 0xae, 0xcc, 0x04, 0x2b, 0x5a, 0xc5, 0x0d, 0x80, 0xea, 0x36, 0x4e,
//         0xd8, 0x41, 0xf8, 0xfc, 0x7a, 0xb3, 0x59, 0x91, 0x9f, 0xea, 0xd9, 0xc9, 0xfa, 0x31, 0xd8,
//         0x94, 0xa9, 0xdd, 0xd8, 0xd9, 0xd8, 0x9d, 0x7d, 0xb5, 0x7d, 0x55, 0x40, 0x1b, 0x31, 0xe5,
//         0x1a, 0x16, 0x9f, 0xa1, 0x1c, 0x33, 0x34, 0x58, 0x58, 0xec, 0x97, 0x31, 0x18, 0x70, 0x71,
//         0xf6, 0x4d, 0xd7, 0x11, 0xda, 0x48, 0x3c, 0xb2, 0x04, 0x0f,
//     ];
//     let deserializer = FullParser::new();
//     let message = deserializer.deserialize(&buffer).unwrap();

//     println!("{:?}", message);
// }

fn main() {
    let config = get_config();

    init_log(config.verbose_level).unwrap();
    let data = if let Some(file_data) = config.file {
        let mut data = Vec::new();
        let mut f = File::open(file_data).expect("Something went wrong reading the file");
        f.read_to_end(&mut data).expect("Failed to read data");
        data
    } else if let Some(data) = config.data {
        data
    } else {
        println!("Unable to parse 'data' value");
        std::process::exit(1);
    };

    let parser = FullParser::new();
    let message = parser.deserialize(&data).unwrap();
    for field in message.fields.iter() {
        println!("{}", field.repr());
    }

    for (i, field) in message.fields.iter().enumerate() {
        println!("{}", field.to_str(&format!("param{}", i)));
    }
}
