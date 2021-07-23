pub mod parser; 
pub mod proto; 

use proto::utils;
use proto::field::*;
use proto::message::*;

use parser::parser::SimpleParser;

fn main() {
    let mut s = Int64Field::default();
    let mut k = Int32Field::default();
    

    println!("Hello, world! {:?}", utils::encode_zigzag_s64(1));
    println!("Hello, world! {:?}", utils::encode_zigzag_s64(-21474836474));
    println!("Hello, world! {:?}", utils::encode_zigzag_s32(2147483647));
    println!("Hello, world! {:?}", utils::encode_zigzag_s32(-2147483648));

    println!("Hello, world! {:?}", utils::decode_zigzag_s64(utils::encode_zigzag_s64(-21474836474)));

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
    
    let mut z = EmbeddedField::<StringField>::default();
    let mut k = EmbeddedField::<StringField>::default();
    
    z.0.data = s;

    println!("{:?}", z.serialize());
    k.deserialize(&z.serialize()).unwrap();
    println!("{:?}", k.0.data);
    

    let mut x = Message::new("mak".to_string(), None);
    x.fields.push(Box::new(k.clone()));
    x.fields.push(Box::new(z.clone()));
    x.fields.push(Box::new(l.clone()));
    println!("{:?}", k.serialize());
    println!("{:?}", z.serialize());
    println!("{:?}", l.serialize());
    println!("{:?}", x.serialize());

    let parser = SimpleParser::new();

    let (x, i) = parser.try_deserialize_specific_field(&l.serialize(), FieldType::String).unwrap();
    let b: &StringField = match x.as_any().downcast_ref::<StringField>() {
        Some(b) => b,
        None => panic!("&a isn't a B!")
    };   
    println!("{:?}", b);

}
