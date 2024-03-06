use quick_error::quick_error;
use rustler::Error as NifError;
use serde::{de, ser};
use std::fmt::Display;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DeserializationError(err: String) {
            display("{}", err)
        }
        TypeHintsRequired {
            display("Cannot deserialize any, type hints are required")
        }
        InvalidAtom {
            display("Failed to deserialize atom")
        }
        InvalidBoolean {
            display("Failed to deserialize boolean")
        }
        InvalidInteger {
            display("Failed to deserialize integer")
        }        
        InvalidFloat {
            display("Failed to deserialize float")
        }
        InvalidStringable {
            display("Failed to deserialize term as an &str")
        }
        InvalidList {
            display("Failed to deserialize list")
        }
        InvalidTuple {
            display("Failed to deserialize tuple")
        }
        InvalidSequenceElement {
            display("Failed to deserialize sequence element")
        }

        ExpectedAtom {
            display("Expected to deserialize atom")
        }
        ExpectedBoolean {
            display("Expected to deserialize boolean")
        }
        ExpectedBinary {
            display("Expected to deserialize binary")
        }
        ExpectedInteger {
            display("Expected to deserialize integer")
        }
        ExpectedFloat {
            display("Expected to deserialize float")
        }
        ExpectedChar {
            display("Expected to deserialize char")
        }
        ExpectedStringable {
            display("Expected to deserialize a UTF-8 stringable term")
        }
        ExpectedNil {
            display("Expected to deserialize nil")
        }
        ExpectedList {
            display("Expected to deserialize list")
        }
        ExpectedTuple {
            display("Expected to deserialize tuple")
        }
        ExpectedEnum {
            display("Expected to deserialize enum")
        }
        ExpectedMap {
            display("Expected to deserialize map")
        }
        ExpectedStruct {
            display("Expected to deserialize struct")
        }
        ExpectedStructName {
            display("Expected to deserialize struct name")
        }
        ExpectedStructValue {
            display("Expected to deserialize struct value")
        }
        ExpectedUnitVariant {
            display("Expected to deserialize unit variant")
        }
        ExpectedNewtypeStruct {
            display("Expected to deserialize newtype struct tuple")
        }
        ExpectedNewtypeVariant {
            display("Expected to deserialize newtype variant")
        }
        ExpectedTupleVariant {
            display("Expected to deserialize tuple variant")
        }
        ExpectedStructVariant {
            display("Expected to deserialize struct variant")
        }

        SerializationError(err: String) {
            display("{}", err)
        }
        InvalidVariantName {
            display("Failed to serialize variant to atom or string")
        }
        InvalidStructName {
            display("Failed to serialize struct name to atom or string")
        }
        InvalidBinary {
            display("Failed to serialize binary")
        }
        InvalidMap {
            display("Failed to serialize map to NIF map")
        }
        InvalidStruct {
            display("Failed to serialize struct to NIF struct")
        }
        InvalidStructKey {
            display("Failed to serialize struct key")
        }
    }
}

impl From<Error> for NifError {
    fn from(err: Error) -> NifError {
        NifError::RaiseTerm(Box::new(err.to_string()))
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::SerializationError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Error {
        Error::DeserializationError(msg.to_string())
    }
}
