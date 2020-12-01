#[cfg(test)]
mod handshake_message_certificate_verify_test;

use super::*;
use crate::signature_hash_algorithm::*;

use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use util::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct HandshakeMessageCertificateVerify {
    pub(crate) hash_algorithm: HashAlgorithm,
    pub(crate) signature_algorithm: SignatureAlgorithm,
    pub(crate) signature: Vec<u8>,
}

const HANDSHAKE_MESSAGE_CERTIFICATE_VERIFY_MIN_LENGTH: usize = 4;

impl HandshakeMessageCertificateVerify {
    pub fn handshake_type(&self) -> HandshakeType {
        HandshakeType::CertificateVerify
    }

    pub fn size(&self) -> usize {
        1 + 1 + 2 + self.signature.len()
    }

    pub fn marshal<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8(self.hash_algorithm as u8)?;
        writer.write_u8(self.signature_algorithm as u8)?;
        writer.write_u16::<BigEndian>(self.signature.len() as u16)?;
        writer.write_all(&self.signature)?;

        Ok(())
    }

    pub fn unmarshal<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let hash_algorithm = reader.read_u8()?.into();
        let signature_algorithm = reader.read_u8()?.into();
        let signature_length = reader.read_u16::<BigEndian>()? as usize;
        let mut signature = vec![0; signature_length];
        reader.read_exact(&mut signature)?;

        Ok(HandshakeMessageCertificateVerify {
            hash_algorithm,
            signature_algorithm,
            signature,
        })
    }
}