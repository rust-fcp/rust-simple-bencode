# rust-simple-bencode
Simple bencode encoder and decoder, that uses neither rustc-serialize or Serde.
Instead, it serializes from / deserializes to a tree using a 4-branch enum."

## Basic usage

This library is mainly composed of the `read` and `write` function, which serialize from and deserialize to this enum:

```rust
pub enum Value {
    String(Vec<u8>),
    Integer(i64),
    List(Vec<Value>),
    Dictionary(HashMap<Vec<u8>, Value>),
}
```

The signature of these functions are:

```rust
pub fn read<R: io::Read>(bytes: &mut Peekable<io::Bytes<R>>) -> Result<Value, DecodeError>;
pub fn write<W: io::Write>(v: &Value, writer: &mut W) -> Result<(), io::Error>;
```

where `DecodeError` is defined like this:

```rust
pub enum DecodeError {
    IOError(io::Error),
    UnexpectedEndOfBuffer,
    UnexpectedCharacter(String)
}
```

## Working with byte arrays

If you work with byte arrays, these shortcuts should be useful to you:

```rust
pub fn decode(sl: &[u8]) -> Result<Value, DecodeError>;
pub fn encode(v: &Value) -> Vec<u8>;
```

## Helpers to deal with `Value`

Because some operations to read `Value` are common, this library provides helpers to avoid error-handling boilerplate:

```rust
pub enum HelperDecodeError {
    BencodeDecodeError(DecodeError),
    BadType(String),
    MissingKey(String),
    FromUtf8Error(FromUtf8Error),
}

/// Pops a BValue::Integer from a HashMap.
pub fn pop_value_integer(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<i64, HelperDecodeError>;

/// Pops a BValue::String from a HashMap.
pub fn pop_value_bytestring(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Vec<u8>, HelperDecodeError>;

/// Pops an optional BValue::String from a HashMap.
pub fn pop_value_bytestring_option(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Option<Vec<u8>>, HelperDecodeError>;

/// Pops a BValue::String from a HashMap and decode it into a Rust String.
pub fn pop_value_utf8_string(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<String, HelperDecodeError>;

/// Pops an optional BValue::String from a HashMap and decode it into a Rust String.
pub fn pop_value_utf8_string_option(map: &mut HashMap<Vec<u8>, Value>, key: String) -> Result<Option<String>, HelperDecodeError>;
```
