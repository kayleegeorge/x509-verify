//! Verification Errors

use core::convert::Infallible;
use der::asn1::ObjectIdentifier;

#[derive(Clone, Debug)]
pub enum Error {
    /// Verification Error
    Verification,

    /// Invalid Key
    InvalidKey,

    /// Invalid Signature
    InvalidSignature,

    /// Unknown OID
    UnknownOid(ObjectIdentifier),

    /// Decoding Error
    Decode,

    /// Encoding Error
    Encode,
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
