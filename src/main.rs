pub mod parser;
pub mod proto;

pub mod args;
pub mod log;


use proto::field::*;
use proto::message::*;
use proto::utils;

use parser::parser::{Parser, SimpleParser, FullParser};

use args::get_config;
use crate::log::{Logger, init_with_level};

fn test() {
    let mut s = Int64Field::default();
    let mut k = Int32Field::default();

    println!("Hello, world! {:?}", utils::encode_zigzag_s64(1));
    println!("Hello, world! {:?}", utils::encode_zigzag_s64(-21474836474));
    println!("Hello, world! {:?}", utils::encode_zigzag_s32(2147483647));
    println!("Hello, world! {:?}", utils::encode_zigzag_s32(-2147483648));

    println!(
        "Hello, world! {:?}",
        utils::decode_zigzag_s64(utils::encode_zigzag_s64(-21474836474))
    );

    s.0.data = 0xFFFFFFFF;
    s.0.number = 1212;
    let x = utils::serialize_varint(150);

    println!("Hello, world! {:?}", utils::deserialize_varint(&x).unwrap());
    println!("Hello, world! {:?}", s.serialize());
    k.deserialize(&s.serialize()).unwrap();
    println!("Hello, world! {:?}", k.0.data);

    let mut l = StringField::default();
    let mut s = StringField::default();
    l.0.data = "Maksim".to_string();

    println!("{:?}", l.serialize());
    s.deserialize(&l.serialize());
    println!("{:?}", l.0.data);

    let mut ms = Message::new("mak".to_string(), None);
    ms.fields.push(Box::new(l.clone()));
    ms.fields.push(Box::new(l.clone()));
    ms.fields.push(Box::new(l.clone()));
    println!("{:?}", k.serialize());
    println!("{:?}", l.serialize());
    println!("{:?}", ms.serialize());

    let parser = SimpleParser::new();

    let (mut x, i) = parser
        .try_deserialize_specific_field(&l.serialize(), FieldType::String)
        .unwrap();
    let b: &StringField = match x.as_any().downcast_ref::<StringField>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };
    println!("{:?}", b);

    let (mut x, i) = parser.try_deserialize_field(&l.serialize()).unwrap();
    let b: &StringField = match x.as_any().downcast_ref::<StringField>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };
    println!("{:?}", b);

    let mut message = parser.deserialize(&ms.serialize()).unwrap();
    for field in message.fields.iter_mut() {
        let b: &StringField = match field.as_any().downcast_ref::<StringField>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };
        println!("{:?}", b);
    }
}

use std::fs::File;
use std::io::Read;

// fn main() {
//     let config = get_config();
//     let mut data = Vec::new();

//     Logger::new().with_level(config.verbose_level).init().unwrap();

//     let mut f = File::open(config.file).expect("Something went wrong reading the file");
//     f.read_to_end(&mut data).expect("Failed to read data");

//     let parser = SimpleParser::new();
//     let message = parser.deserialize(&data).unwrap();
 
//     for field in message.fields.iter() {
//         // let b: &StringField = match field.as_any().downcast_ref::<StringField>() {
//         //     Some(b) => b,
//         //     None => panic!("&a isn't a B!"),
//         // };
//         println!("{}", field.repr());
//     }
// }

fn main() {
    let config = get_config();
    let mut data = Vec::new();

    Logger::new().with_level(config.verbose_level).init().unwrap();

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

