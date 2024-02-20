use crate::{
    yaml_parser::{CheckRootHash, Contains, Get, Insert, Remove},
    Command, CommandList,
};

use super::CommandId;
use num_bigint::{BigUint, RandBigInt};
use rand::prelude::*;
use strum::EnumCount;

// represented as a big-endian byte array becaute BigUint as no const builder
// binary tree height is 251 and the max key is 2^{251}
const MAX_KEY: [u8; 32] = [
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
];
// as the max value for felt is 2^{251}+17*2^{192}+1
const MAX_VALUE: [u8; 32] = [
    8, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1,
];

pub fn generate_random_scenario() -> CommandList {
    let mut rng = rand::thread_rng();

    let mut commands: Vec<Command> = Vec::new();
    let num_cmd = rng.gen_range(1..=40);

    for _ in 0..num_cmd {
        let id: CommandId =
            CommandId::from_repr(rng.gen_range(0..CommandId::COUNT))
                .expect("Invalid CommandId representation");
        let key = generate_key(&mut rng);
        let value = generate_value(&mut rng);

        // println!("{} {:?} - {} - {}", id, &key, key.len(), &value);

        let command: Command = match id {
            CommandId::Insert => Insert { key, value }.into(),
            CommandId::CheckRootHash => CheckRootHash {
                expected_value: value,
            }
            .into(),
            CommandId::Get => Get {
                key,
                expected_value: value,
            }
            .into(),
            CommandId::Contains => Contains {
                key,
                expected_value: rng.gen::<bool>(),
            }
            .into(),
            CommandId::Remove => Remove { key }.into(),
        };
        commands.push(command);
    }

    commands.into()
}

// generate a random valid key that is less than 2^{251} and > 0
fn generate_key(rng: &mut ThreadRng) -> Vec<u8> {
    let max_key = BigUint::from_bytes_be(&MAX_KEY);
    loop {
        let key = rng.gen_biguint(max_key.bits());
        if key <= max_key && key > BigUint::from(0u8) {
            break key.to_bytes_be();
        }
    }
}
fn generate_value(rng: &mut ThreadRng) -> String {
    let max_value = BigUint::from_bytes_be(&MAX_VALUE);

    let value = loop {
        let value = rng.gen_biguint(max_value.bits());
        if value <= max_value {
            break value;
        }
    };

    format!(
        "0x{}",
        value
            .to_bytes_be()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_scenario() {
        generate_random_scenario();
    }
}
