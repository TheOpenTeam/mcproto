use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::ops::{Index, IndexMut};

use crate::NbtError;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(HashMap<String, NbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug, Clone)]
pub struct Nbt {
    pub root: HashMap<String, NbtValue>,
}
impl Index<&str> for Nbt {
    type Output = NbtValue;

    fn index(&self, path: &str) -> &Self::Output {
        self.try_get(path).expect("path not found")
    }
}

impl IndexMut<&str> for Nbt {
    fn index_mut(&mut self, path: &str) -> &mut Self::Output {
        self.try_get_mut(path).expect("path not found")
    }
}
impl Nbt {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NbtError> {
        let mut cursor = Cursor::new(bytes);
        let tag_id = read_u8(&mut cursor)?;
        if tag_id != 10 {
            return Err(NbtError::InvalidRoot);
        }
        let _name = read_string(&mut cursor)?;
        let root = read_compound(&mut cursor)?;
        Ok(Self { root })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, NbtError> {
        let mut buf = Vec::new();
        write_u8(&mut buf, 10)?;
        write_string(&mut buf, "")?;

        write_compound(&mut buf, &self.root)?;

        Ok(buf)
    }
    // 输入路径
    pub fn try_get<T: AsRef<str>>(&self, path: T) -> Option<&NbtValue> {
        let mut parts = path.as_ref().split('.');
        let current = parts.next()?;
        let mut value = self.root.get(current)?;

        for key in parts {
            match value {
                NbtValue::Compound(map) => {
                    value = map.get(key)?;
                }
                _ => return None,
            }
        }
        Some(value)
    }

    pub fn try_get_mut<T: AsRef<str>>(&mut self, path: T) -> Option<&mut NbtValue> {
        let mut parts = path.as_ref().split('.');
        let current = parts.next()?;
        let mut value = self.root.get_mut(current)?;

        for key in parts {
            match value {
                NbtValue::Compound(map) => {
                    value = map.get_mut(key)?;
                }
                _ => return None,
            }
        }
        Some(value)
    }

}

fn read_u8(r: &mut impl Read) -> Result<u8, NbtError> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf).map_err(|e| NbtError::Io(e.to_string()))?;
    Ok(buf[0])
}

fn read_i8(r: &mut impl Read) -> Result<i8, NbtError> {
    Ok(read_u8(r)? as i8)
}

fn read_i16(r: &mut impl Read) -> Result<i16, NbtError> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf).map_err(|e| NbtError::Io(e.to_string()))?;
    Ok(i16::from_be_bytes(buf))
}

fn read_i32(r: &mut impl Read) -> Result<i32, NbtError> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf).map_err(|e| NbtError::Io(e.to_string()))?;
    Ok(i32::from_be_bytes(buf))
}

fn read_i64(r: &mut impl Read) -> Result<i64, NbtError> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf).map_err(|e| NbtError::Io(e.to_string()))?;
    Ok(i64::from_be_bytes(buf))
}

fn read_f32(r: &mut impl Read) -> Result<f32, NbtError> {
    Ok(f32::from_bits(read_i32(r)? as u32))
}

fn read_f64(r: &mut impl Read) -> Result<f64, NbtError> {
    Ok(f64::from_bits(read_i64(r)? as u64))
}

fn read_string(r: &mut impl Read) -> Result<String, NbtError> {
    let len = read_i16(r)? as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).map_err(|e| NbtError::Io(e.to_string()))?;
    String::from_utf8(buf).map_err(|e| NbtError::Utf8(e.to_string()))
}

fn read_value(r: &mut impl Read, tag: u8) -> Result<NbtValue, NbtError> {
    Ok(match tag {
        1 => NbtValue::Byte(read_i8(r)?),
        2 => NbtValue::Short(read_i16(r)?),
        3 => NbtValue::Int(read_i32(r)?),
        4 => NbtValue::Long(read_i64(r)?),
        5 => NbtValue::Float(read_f32(r)?),
        6 => NbtValue::Double(read_f64(r)?),

        7 => {
            let len = read_i32(r)? as usize;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                v.push(read_i8(r)?);
            }
            NbtValue::ByteArray(v)
        }

        8 => NbtValue::String(read_string(r)?),

        9 => {
            let inner_tag = read_u8(r)?;
            let len = read_i32(r)? as usize;
            let mut list = Vec::with_capacity(len);
            for _ in 0..len {
                list.push(read_value(r, inner_tag)?);
            }
            NbtValue::List(list)
        }

        10 => NbtValue::Compound(read_compound(r)?),

        11 => {
            let len = read_i32(r)? as usize;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                v.push(read_i32(r)?);
            }
            NbtValue::IntArray(v)
        }

        12 => {
            let len = read_i32(r)? as usize;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                v.push(read_i64(r)?);
            }
            NbtValue::LongArray(v)
        }

        _ => return Err(NbtError::UnknownTag(tag)),
    })
}

fn read_compound(r: &mut impl Read) -> Result<HashMap<String, NbtValue>, NbtError> {
    let mut map = HashMap::new();

    loop {
        let tag = read_u8(r)?;
        if tag == 0 {
            break;
        }

        let name = read_string(r)?;
        let value = read_value(r, tag)?;
        map.insert(name, value);
    }

    Ok(map)
}


fn write_u8(w: &mut Vec<u8>, v: u8) -> Result<(), NbtError> {
    w.push(v);
    Ok(())
}

fn write_i8(w: &mut Vec<u8>, v: i8) -> Result<(), NbtError> {
    w.push(v as u8);
    Ok(())
}

fn write_i16(w: &mut Vec<u8>, v: i16) -> Result<(), NbtError> {
    w.extend_from_slice(&v.to_be_bytes());
    Ok(())
}

fn write_i32(w: &mut Vec<u8>, v: i32) -> Result<(), NbtError> {
    w.extend_from_slice(&v.to_be_bytes());
    Ok(())
}

fn write_i64(w: &mut Vec<u8>, v: i64) -> Result<(), NbtError> {
    w.extend_from_slice(&v.to_be_bytes());
    Ok(())
}

fn write_f32(w: &mut Vec<u8>, v: f32) -> Result<(), NbtError> {
    write_i32(w, v.to_bits() as i32)
}

fn write_f64(w: &mut Vec<u8>, v: f64) -> Result<(), NbtError> {
    write_i64(w, v.to_bits() as i64)
}

fn write_string(w: &mut Vec<u8>, s: &str) -> Result<(), NbtError> {
    write_i16(w, s.len() as i16)?;
    w.extend_from_slice(s.as_bytes());
    Ok(())
}

fn write_value(w: &mut Vec<u8>, v: &NbtValue) -> Result<u8, NbtError> {
    match v {
        NbtValue::Byte(x) => {
            write_i8(w, *x)?;
            Ok(1)
        }
        NbtValue::Short(x) => {
            write_i16(w, *x)?;
            Ok(2)
        }
        NbtValue::Int(x) => {
            write_i32(w, *x)?;
            Ok(3)
        }
        NbtValue::Long(x) => {
            write_i64(w, *x)?;
            Ok(4)
        }
        NbtValue::Float(x) => {
            write_f32(w, *x)?;
            Ok(5)
        }
        NbtValue::Double(x) => {
            write_f64(w, *x)?;
            Ok(6)
        }
        NbtValue::ByteArray(arr) => {
            write_i32(w, arr.len() as i32)?;
            for v in arr {
                write_i8(w, *v)?;
            }
            Ok(7)
        }
        NbtValue::String(s) => {
            write_string(w, s)?;
            Ok(8)
        }
        NbtValue::List(list) => {
            let tag = if let Some(first) = list.first() {
                write_value(&mut Vec::new(), first)?
            } else {
                1
            };
            write_u8(w, tag)?;
            write_i32(w, list.len() as i32)?;
            for item in list {
                write_value(w, item)?;
            }
            Ok(9)
        }
        NbtValue::Compound(map) => {
            write_compound(w, map)?;
            Ok(10)
        }
        NbtValue::IntArray(arr) => {
            write_i32(w, arr.len() as i32)?;
            for v in arr {
                write_i32(w, *v)?;
            }
            Ok(11)
        }
        NbtValue::LongArray(arr) => {
            write_i32(w, arr.len() as i32)?;
            for v in arr {
                write_i64(w, *v)?;
            }
            Ok(12)
        }
    }
}

fn write_compound(w: &mut Vec<u8>, map: &HashMap<String, NbtValue>) -> Result<(), NbtError> {
    for (name, value) in map {
        let mut tmp = Vec::new();
        let tag = write_value(&mut tmp, value)?;
        write_u8(w, tag)?;
        write_string(w, name)?;
        w.extend_from_slice(&tmp);
    }
    write_u8(w, 0)?;
    Ok(())
}
