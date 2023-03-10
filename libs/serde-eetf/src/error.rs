use std::fmt::{self, Display};

use serde::{de, ser};

/// The Result type of a serialization/deserialization.
pub type Result<T> = std::result::Result<T, Error>;

/// An Error type for serializing/deserializing EETF.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),

    //TODO: DecodeError(eetf::DecodeError),
    DecodeError(String),
    EncodeError(String),
    TypeHintsRequired,
    ExpectedBoolean,
    InvalidBoolean,
    ExpectedFixInteger,
    ExpectedFloat,
    ExpectedChar,
    ExpectedBinary,
    Utf8DecodeError,
    ExpectedNil,
    ExpectedList,
    ExpectedTuple,
    WrongTupleLength,
    ExpectedMap,
    ExpectedAtom,
    IntegerConvertError,
    FloatConvertError,
    TooManyItems,
    MisSizedVariantTuple,
    ExpectedAtomOrTuple,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match *self {
            Error::Message(ref msg) => msg,
            Error::DecodeError(_) => "Decode error",
            Error::EncodeError(ref msg) => msg,
            Error::TypeHintsRequired => "Type Hints are required for deserializing eetf",
            Error::ExpectedBoolean => "Expected boolean, got something else",
            Error::InvalidBoolean => "Invalid boolean",
            Error::ExpectedFixInteger => "Expected fix integer, got something else",
            Error::ExpectedFloat => "Expected float integer, got something else",
            Error::ExpectedChar => "Expected string of one character, got something else",
            Error::ExpectedBinary => "Expected binary, got something else",
            Error::Utf8DecodeError => "Error decoding UTF8 from binary",
            Error::ExpectedNil => "Expected nil, got something else",
            Error::ExpectedList => "Expected list, got something else",
            Error::ExpectedTuple => "Expected tuple, got something else",
            Error::WrongTupleLength => "Tuple was wrong length",
            Error::ExpectedMap => "Expected map, got something else",
            Error::ExpectedAtom => "Expected atom, got something else",
            Error::IntegerConvertError => "Could not convert integer without overflow",
            Error::FloatConvertError => "Could not convert float without overflow",
            Error::TooManyItems => "Too many items when deserializing sequence",
            Error::MisSizedVariantTuple => "Was expecting a tuple of an atom and element",
            Error::ExpectedAtomOrTuple => "Was expecting an atom or a tuple",
        })
    }
}

impl From<eetf::DecodeError> for Error {
    fn from(err: eetf::DecodeError) -> Error {
        Error::DecodeError(err.to_string())
    }
}
