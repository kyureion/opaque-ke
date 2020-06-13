// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::errors::InternalPakeError;

use generic_array::GenericArray;
use sha2::{Digest, Sha256};

pub trait SlowHash {
    fn hash(
        input: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
    ) -> Result<Vec<u8>, InternalPakeError>;
}

pub struct NoOpHash;

impl SlowHash for NoOpHash {
    fn hash(
        input: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
    ) -> Result<Vec<u8>, InternalPakeError> {
        Ok(input.to_vec())
    }
}

#[cfg(feature = "slow-hash")]
impl SlowHash for scrypt::ScryptParams {
    fn hash(
        input: GenericArray<u8, <Sha256 as Digest>::OutputSize>,
    ) -> Result<Vec<u8>, InternalPakeError> {
        let params = scrypt::ScryptParams::new(15, 8, 1).unwrap();
        let mut output = [0u8; 32];
        scrypt::scrypt(&input, &[], &params, &mut output)
            .map_err(|_| InternalPakeError::SlowHashError)?;
        Ok(output.to_vec())
    }
}