// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::algorithms::Poseidon;
use snarkvm_algorithms::crypto_hash::hash_to_curve;
use snarkvm_circuits_environment::prelude::*;
use snarkvm_circuits_types::prelude::*;
use snarkvm_fields::{FieldParameters, PrimeField};

/// ECIESPoseidonEncryption is an encryption gadget which uses Poseidon under the hood.
pub struct ECIESPoseidonEncryption<E: Environment> {
    generator: E::Affine,
    poseidon: Poseidon<E>,
    symmetric_key_commitment_domain: Field<E>,
    symmetric_encryption_domain: Field<E>,
}

impl<E: Environment> ECIESPoseidonEncryption<E> {
    /// Initializes a new instance of the ECIES gadget with the given setup message.
    pub fn setup(message: &str) -> Self {
        let (generator, _, _) = hash_to_curve::<_>(message);
        let poseidon = Poseidon::<E>::new();
        let symmetric_key_commitment_domain =
            Field::constant(E::BaseField::from_bytes_le_mod_order(b"AleoSymmetricKeyCommitment0"));
        let symmetric_encryption_domain =
            Field::constant(E::BaseField::from_bytes_le_mod_order(b"AleoSymmetricEncryption0"));

        Self { generator, poseidon, symmetric_key_commitment_domain, symmetric_encryption_domain }
    }

    /// Encode a bitstring into a vector of field elements. This is used to convert messages
    /// to hashable [`Field`] elements.
    pub fn encode_message(&self, message: &[Boolean<E>]) -> Vec<Field<E>> {
        // Add an extra bit to the message.
        // The final bit serves as a terminus indicator,
        // and is used during decryption to ensure the length is correct.
        let mut bits = message.to_vec();
        bits.push(Boolean::constant(true));

        // Determine the number of ciphertext elements.
        let capacity = <<E::BaseField as PrimeField>::Parameters as FieldParameters>::CAPACITY as usize;

        // Pack the bits into field elements.
        bits.chunks(capacity).map(|chunk| Field::from_bits_le(chunk)).collect()
    }

    /// Decode a vector of field elements to a bitstring. This is used to convert back from
    /// hashable [`Field`] elements to a normal message.
    pub fn decode_message(&self, encoded_message: &[Field<E>]) -> Vec<Boolean<E>> {
        let capacity = <<E::BaseField as PrimeField>::Parameters as FieldParameters>::CAPACITY as usize;

        let mut bits = Vec::<Boolean<E>>::with_capacity(encoded_message.len() * capacity);
        for element in encoded_message.iter() {
            bits.extend_from_slice(&element.to_bits_le()[..capacity]);
        }

        // Drop all the ending zeros and the last "1" bit.
        // Note that there must be at least one "1" bit because the last element is not zero.
        loop {
            if bits.pop().unwrap().eject_value() == true {
                break;
            }
        }

        if bits.len() % 8 != 0 {
            E::halt("The number of bits in the packed field elements is not a multiple of 8.")
        } else {
            bits
        }
    }

    /// Symetrically encrypt a string of plaintext, using a given symmetric key.
    pub fn encrypt(&self, symmetric_key: Field<E>, message: &[Field<E>]) -> Vec<Field<E>> {
        let randomizers =
            self.poseidon.hash_many(&[self.symmetric_encryption_domain.clone(), symmetric_key], message.len());

        message.iter().zip_eq(randomizers).map(|(plaintext, randomizer)| plaintext + randomizer).collect()
    }

    /// Decrypt a ciphertext with the given symmetric key.
    pub fn decrypt(&self, symmetric_key: Field<E>, ciphertext: &[Field<E>]) -> Vec<Field<E>> {
        let randomizers =
            self.poseidon.hash_many(&[self.symmetric_encryption_domain.clone(), symmetric_key], ciphertext.len());

        ciphertext.iter().zip_eq(randomizers).map(|(ciphertext, randomizer)| ciphertext - &randomizer).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_algorithms::encryption::ECIESPoseidonEncryption as NativeECIES;

    #[test]
    fn test_encode() {}
}
