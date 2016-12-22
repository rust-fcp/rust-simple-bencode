use std::io;
use std::collections::HashMap;

use value::Value;

fn write_string<W: io::Write>(s: &Vec<u8>, writer: &mut W) -> Result<(), io::Error> {
    try!(writer.write(&format!("{}:", s.len()).into_bytes()));
    try!(writer.write(s));
    Ok(())
}
fn write_integer<W: io::Write>(i: &i64, writer: &mut W) -> Result<(), io::Error> {
    try!(writer.write(&format!("i{}e", i).into_bytes()));
    Ok(())
}
fn write_list<W: io::Write>(l: &Vec<Value>, writer: &mut W) -> Result<(), io::Error> {
    try!(writer.write(&vec!['l' as u8]));
    for item in l {
        try!(write(item, writer));
    }
    try!(writer.write(&vec!['e' as u8]));
    Ok(())
}
fn write_dictionary<W: io::Write>(d: &HashMap<Vec<u8>, Value>, writer: &mut W) -> Result<(), io::Error> {
    try!(writer.write(&vec!['d' as u8]));
    let mut items: Vec<_> = d.iter().collect();
    items.sort_by_key(|&(k, _v)| k); // Keys of bencode dict must be in alphabetical order.
    for (key, value) in items {
        try!(write_string(key, writer));
        try!(write(value, writer));
    }
    try!(writer.write(&vec!['e' as u8]));
    Ok(())
}

pub fn write<W: io::Write>(v: &Value, writer: &mut W) -> Result<(), io::Error> {
    match *v {
        Value::String(ref s) => write_string(s, writer),
        Value::Integer(ref i) => write_integer(i, writer),
        Value::List(ref l) => write_list(l, writer),
        Value::Dictionary(ref d) => write_dictionary(d, writer),
    }
}

pub fn encode(v: &Value) -> Vec<u8> {
    let mut res = Vec::new();
    write(v, &mut res).unwrap();
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use value::Value;
    use super::*;

    #[test]
    fn integer() {
        assert_eq!("i1234e", String::from_utf8(encode(&Value::Integer(1234))).unwrap());
        assert_eq!("i-1234e", String::from_utf8(encode(&Value::Integer(-1234))).unwrap());
        assert_eq!("i0e", String::from_utf8(encode(&Value::Integer(0))).unwrap());
    }

    #[test]
    fn string() {
        assert_eq!("5:abcde", String::from_utf8(encode(&Value::String(b"abcde".to_vec()))).unwrap());
        assert_eq!("0:", String::from_utf8(encode(&Value::String(b"".to_vec()))).unwrap());
    }

    #[test]
    fn array() {
        assert_eq!("li1234ee", String::from_utf8(encode(&Value::List(vec![Value::Integer(1234)]))).unwrap());
        assert_eq!("li1234ei0ee", String::from_utf8(encode(&Value::List(vec![Value::Integer(1234), Value::Integer(0)]))).unwrap());
    }

    #[test]
    fn dict() {
        let mut expected = HashMap::new();
        expected.insert(b"cow".to_vec(), Value::String(b"moo".to_vec()));
        expected.insert(b"spam".to_vec(), Value::String(b"eggs".to_vec()));
        assert_eq!("d3:cow3:moo4:spam4:eggse", String::from_utf8(encode(&Value::Dictionary(expected))).unwrap());
    }
}
