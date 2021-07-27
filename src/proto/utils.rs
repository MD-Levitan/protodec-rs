use crate::proto::error::Result;

/// Serialization using Varints method
pub fn serialize_varint(var: u64) -> Vec<u8> {
    let mut gen = Vec::new();
    serialize_varint_into(var, &mut gen);

    gen
}

/// Serialization using Varints method into Vec
pub fn serialize_varint_into(var: u64, gen: &mut Vec<u8>) {
    let mut x = var;
    match x {
        0 => gen.push(0),
        _ => {
            while x != 0 {
                gen.push((((x & 0x7F) as u8) | (((x >> 7) != 0) as u8) << 0x7) as u8);
                x = x >> 7;
            }
        }
    }
}

/// Deserialization using Varints method
///
/// Returns (result, bytes readed)
use std::ops::Add;
pub fn deserialize_varint(gen: &[u8]) -> Result<(u64, u64)> {
    let mut result: u64 = 0;
    let mut readed: u64 = 0;
    for (i, x) in gen.iter().enumerate() {
        result |= ((x & 0x7F) as u64) << (i * 7);
        if x >> 7 == 0 {
            readed = (i + 1) as u64;
            break;
        }
    }
    log::trace!(
        "VarInt: bytes {} -> <result {}[{}], {}[{}]>",
        &gen[0..readed as usize]
            .iter()
            .fold(String::new(), |s, x| s.add(&format!("{:02x} ", x))),
        result,
        result,
        readed,
        readed,
    );
    Ok((result, readed))
}

/// Generate key using next alg: (field_number << 3) | wire_type
pub fn generate_key(field_number: u64, wire_type: u8) -> u64 {
    ((field_number & 0x1FFFFFFFFFFFFFFF) << 3) | (wire_type as u64)
}

/// Parse key using next alg: (field_number << 3) | wire_type
pub fn parse_key(key: u64) -> (u64, u8) {
    (key >> 3, (key & 0x7) as u8)
}

/// ZigZag Encoding for sint32
pub fn encode_zigzag_s32(var: i32) -> u64 {
    match (var as u32) >> 31 {
        0 => (((var as u32) << 1) ^ ((var as u32) >> 31)) as u64,
        _ => ((((var as u32) ^ 0x7FFFFFFF) << 1) ^ ((var as u32) >> 31)) as u64,
    }
}

/// ZigZag Decoding for sint32
pub fn decode_zigzag_s32(var: u64) -> i32 {
    match var & 0x1 {
        0 => ((var << 31) ^ (var >> 1)) as i32,
        _ => ((var << 31) ^ (var >> 1) ^ 0x7FFFFFFF) as i32,
    }
}

/// ZigZag Encoding for sint64
pub fn encode_zigzag_s64(var: i64) -> u64 {
    match (var as u64) >> 63 {
        0 => ((var as u64) << 1) ^ ((var as u64) >> 63),
        _ => (((var as u64) ^ 0x7FFFFFFFFFFFFFFF) << 1) ^ ((var as u64) >> 63),
    }
}

/// ZigZag Decoding for sint64
pub fn decode_zigzag_s64(var: u64) -> i64 {
    match var & 0x1 {
        0 => ((var << 63) ^ (var >> 1)) as i64,
        _ => ((var << 63) ^ (var >> 1) ^ 0x7FFFFFFFFFFFFFFF) as i64,
    }
}
