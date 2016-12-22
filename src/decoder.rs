use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::collections::HashMap;

use value::Value;

#[derive(Debug)]
pub enum DecodeError {
    IOError(io::Error),
    UnexpectedEndOfBuffer,
    UnexpectedCharacter(String)
}

macro_rules! try_read {
    ($e: expr) => (
        match $e.next() {
            None => return Err(DecodeError::UnexpectedEndOfBuffer),
            Some(Err(e)) => return Err(DecodeError::IOError(e)),
            Some(Ok(c)) => c,
        }
    );
}
macro_rules! try_peek {
    ($e: expr) => ({
        let tmp = match $e.peek() {
            None => return Err(DecodeError::UnexpectedEndOfBuffer),
            Some(&Err(_)) => {
                // We need an owned version of IOError. If peek() raised one,
                // hopefully, next() will, so let's do it.
                // Unfortunately, we cannot do it now because the reader is
                // still mutably borrowed, so let's defer the call.
                None
            }
            Some(&Ok(c)) => Some(c),
        };
        match tmp {
            Some(c) => c,
            None => return Err(DecodeError::IOError($e.next().unwrap().unwrap_err()))
        }
    });
}

fn read_integer<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>) -> Result<i64, DecodeError> {
    let mut res = 0i64;
    let first_digit = try_peek!(bytes);
    let multiplicator = if first_digit as char == '-' { try_read!(bytes); -1 } else { 1 };
    loop {
        let digit = try_read!(bytes);
        match digit as char {
            'e' => break,
            '0' ... '9' => res = res*10 + (digit as i64 - ('0' as i64)),
            _ => return Err(DecodeError::UnexpectedCharacter(format!("'{}' while reading an integer.", digit as char))),
        }
    };
    Ok(multiplicator * res)
}

fn read_list<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>) -> Result<Vec<Value>, DecodeError> {
    let mut res = Vec::<Value>::new();
    loop {
        let digit = try_peek!(bytes);
        match digit as char {
            'e' => break,
            _ => res.push(try!(read(bytes))),
        }
    }
    Ok(res)
}

fn read_string<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>, first_byte: u8) -> Result<Vec<u8>, DecodeError> {
    assert!(first_byte >= '0' as u8);
    assert!(first_byte <= '9' as u8);
    let mut length = first_byte as usize - ('0' as usize);
    loop {
        let digit = try_read!(bytes);
        match digit as char {
            ':' => break,
            '0' ... '9' => length = length*10 + digit as usize - ('0' as usize),
            _ => return Err(DecodeError::UnexpectedCharacter(format!("'{}' while reading a string length", digit as char)))
        }
    }
    let mut res = Vec::new();
    res.reserve(length);
    for _ in 0..length {
        res.push(try_read!(bytes));
    }
    Ok(res)
}

fn read_dict<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>) -> Result<HashMap<Vec<u8>, Value>, DecodeError> {
    let mut res = HashMap::<Vec<u8>, Value>::new();
    loop {
        let first_byte = try_read!(bytes);
        if first_byte as char == 'e' {
            break
        }
        res.insert(try!(read_string(bytes, first_byte)), try!(read(bytes)));
    }
    Ok(res)
}


pub fn read<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>) -> Result<Value, DecodeError> {
    let byte = try_read!(bytes);
    match byte as char {
        'i' => read_integer(bytes).map(Value::Integer),
        'l' => read_list(bytes).map(Value::List),
        'd' => read_dict(bytes).map(Value::Dictionary),
        '0' ... '9' => read_string(bytes, byte).map(Value::String),
        _ => Err(DecodeError::UnexpectedCharacter(format!("'{}' instead of the first byte of an object.", byte)))
    }
}

pub fn decode(sl: &[u8]) -> Result<Value, DecodeError> {
    read(&mut sl.bytes().peekable())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use value::Value;
    use super::*;

    #[test]
    fn integer() {
        assert_eq!(decode(b"i1234e").unwrap(), Value::Integer(1234));
        assert_eq!(decode(b"i1234eaaaa").unwrap(), Value::Integer(1234));
        assert_eq!(decode(b"i-1234e").unwrap(), Value::Integer(-1234));
        assert_eq!(decode(b"i0e").unwrap(), Value::Integer(0));
        assert!(decode(b"i12a34e").is_err());
    }

    #[test]
    fn string() {
        assert_eq!(decode(b"5:abcde").unwrap(), Value::String(b"abcde".to_vec()));
        assert_eq!(decode(b"5:abcdefg").unwrap(), Value::String(b"abcde".to_vec()));
        assert_eq!(decode(b"0:").unwrap(), Value::String(b"".to_vec()));
        assert!(decode(b"-1:").is_err());
    }

    #[test]
    fn array() {
        assert_eq!(decode(b"li1234ee").unwrap(), Value::List(vec![Value::Integer(1234)]));
        assert_eq!(decode(b"li1234ei0ee").unwrap(), Value::List(vec![Value::Integer(1234), Value::Integer(0)]));
    }

    #[test]
    fn dict() {
        let mut expected = HashMap::new();
        expected.insert(b"cow".to_vec(), Value::String(b"moo".to_vec()));
        expected.insert(b"spam".to_vec(), Value::String(b"eggs".to_vec()));
        assert_eq!(decode(b"d3:cow3:moo4:spam4:eggse").unwrap(), Value::Dictionary(expected));
    }
}
