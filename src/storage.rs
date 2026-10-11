// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::{fs, io};

use sha1::{Digest, Sha1};

use crate::config::StorageConfig;

// Ensures the base storage directory exists on the filesystem.
pub fn init(cfg: &StorageConfig) -> io::Result<()> {
    fs::create_dir_all(&cfg.dir)
}

// Computes the SHA-1 checksum of the given byte slice as a lowercase hexadecimal string.
pub fn checksum(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
