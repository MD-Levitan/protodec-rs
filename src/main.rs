pub mod parser; 

use parser::utils;
use parser::field::*;

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
    let mut k = EmbeddedField::<Int32Field>::default();
    
    z.0.data = s;

    println!("{:?}", z.serialize());
    k.deserialize(&z.serialize()).unwrap();
    println!("{:?}", k.0.data);
    
}
