use std::{error, fmt};

#[derive(Debug)]
pub enum HydroError {
    InvalidInput,
    InvalidKey,
    InvalidPadding,
    InvalidProbe,
    InvalidSignature,
    DecryptionError,
    InitError,
    UnsupportedOutputLength,
}

impl fmt::Display for HydroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use HydroError::*;

        match self {
            InvalidInput => write!(f, "Invalid input"),
            InvalidKey => write!(f, "Invalid key"),
            InvalidPadding => write!(f, "Invalid padding"),
            InvalidProbe => write!(f, "Invalid probe"),
            InvalidSignature => write!(f, "Invalid signature"),
            DecryptionError => write!(f, "Unable to decrypt the ciphertext"),
            InitError => write!(f, "Unable to initialize the hydrogen library"),
            UnsupportedOutputLength => write!(f, "Unsupported output length"),
        }
    }
}

impl error::Error for HydroError {}
