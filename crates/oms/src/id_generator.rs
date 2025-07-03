// Copyright 2025 chenjjiaa
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused)]

use once_cell::sync::Lazy;
use rand::RngCore;
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::id_generator;

#[derive(Debug, Clone, Copy)]
pub struct Uint128(pub [u8; 16]);

impl fmt::Display for Uint128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// The type from &str -> Uint128
impl FromStr for Uint128 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 32 {
            return Err("Invalid length for Uint128 hex string");
        }

        let mut bytes = [0u8; 16];
        for i in 0..16 {
            let byte_str = &s[2 * i..2 * i + 2];
            bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex")?;
        }
        Ok(Uint128(bytes))
    }
}

static ID_STATE: Lazy<Mutex<IdState>> = Lazy::new(|| Mutex::new(IdState::new()));

struct IdState {
    last_timestamp: u128,
    last_random: [u8; 10],
}

impl IdState {
    fn new() -> Self {
        IdState {
            last_timestamp: 0,
            last_random: [0; 10],
        }
    }
}

pub fn generate_id() -> Uint128 {
    let mut state = ID_STATE.lock().unwrap();

    // Get current timestamp in nanoseconds
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    let timestamp = if now <= state.last_timestamp {
        state.last_timestamp
    } else {
        state.last_timestamp = now;
        rand::rng().fill_bytes(&mut state.last_random);
        now
    };

    // Treat last_random as a u80 made of [u64, u16]
    let (mut low, mut high): (u64, u16) = {
        let low = u64::from_le_bytes(state.last_random[0..8].try_into().unwrap());
        let high = u16::from_le_bytes(state.last_random[8..10].try_into().unwrap());
        (low, high)
    };

    // Increment as uint80
    low = low.wrapping_add(1);
    if low == 0 {
        high = high.wrapping_add(1);
        if high == 0 {
            panic!("random bits overflow on monotonic increment");
        }
    }

    // Write back
    state.last_random[0..8].copy_from_slice(&low.to_le_bytes());
    state.last_random[8..10].copy_from_slice(&high.to_le_bytes());

    drop(state); // release the lock

    // Compose the final ID (little-endian): [0..10] random, [10..16] timestamp
    let mut id = [0u8; 16];
    id[0..8].copy_from_slice(&low.to_le_bytes());
    id[8..10].copy_from_slice(&high.to_le_bytes());

    // Split timestamp into lower 16 bits and higher 32 bits
    let lo = (timestamp & 0xFFFF) as u16;
    let hi = ((timestamp >> 16) & 0xFFFF_FFFF) as u32;

    id[10..12].copy_from_slice(&lo.to_le_bytes());
    id[12..16].copy_from_slice(&hi.to_le_bytes());

    Uint128(id)
}

/// ``` rust
/// let id = generate_id();
/// println!("ID: {:02x?}", id.0);
/// println!("ID: {:?}", id.to_string());
/// ```
#[test]
fn foo() {
    for _ in 0..10 {
        let id = generate_id();
        println!("{}", id);
    }

    let id_ = generate_id();
    println!("id_: {:?}", id_);
    let s = id_.to_string();
    let i: Uint128 = s.parse().unwrap();
    println!("i  : {:?}", i);
}
