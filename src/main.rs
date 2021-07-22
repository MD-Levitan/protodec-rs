pub mod parser; 

use parser::utils;
use parser::field::*;

fn main() {
    let mut s = Int32Field::default();
    let mut k = Int32Field::default();
    
    s.0.data = 33;
    s.0.number = 1212;
    let x = utils::serialize_varint(150);
    println!("Hello, world! {:?}", x);

    println!("Hello, world! {:?}", utils::deserialize_varint(&x).unwrap());
    //println!("Hello, world! {:?}", s.serialize());
    k.deserialize(&s.serialize());
    println!("Hello, world! {:?}", k.0.data);

}
