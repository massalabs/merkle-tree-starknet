use std::{collections::HashMap, ffi::CString, fs, path::PathBuf};
mod command_line;
use crate::{command::CommandTrait, tree::TestTree};
use anyhow::Result;
use command_line::Cli;
use rust_ffi::{Command, CommandId};
use starknet_types_core::felt::Felt;

extern crate log;

pub mod command;
pub mod tree;

pub fn init_runner<'a, F: Fn() -> Box<dyn TestTree>>(
    test_env_creator: F,
) -> HashMap<String, String> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = Cli::parse();

    let mut result = std::collections::HashMap::new();

    if let Some(dir_path) = config.scenario_dir {
        let scenario_dir = fs::read_dir(dir_path).unwrap();

        for entry in scenario_dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                start_test(
                    path,
                    entry.file_name().to_str().unwrap(),
                    &mut result,
                    &test_env_creator,
                );
            }
        }
    } else if let Some(file_path) = config.scenario_file {
        let path = file_path.clone();
        let filename = path.file_name().unwrap().to_str().unwrap();
        start_test(file_path, filename, &mut result, test_env_creator);
    }
    result
}

fn start_test<'a, F: Fn() -> Box<dyn TestTree>>(
    path: PathBuf,
    filename: &str,
    result: &mut HashMap<String, String>,
    test_env_creator: F,
) {
    let mut test_tree = test_env_creator();

    let path_str = path.to_str().unwrap();

    let c_string = CString::new(path_str).expect("Failed to create CString");
    let scenario = c_string.into_raw();
    log::info!("Load scenario {:?}", &filename);
    let command_list = rust_ffi::load_scenario(scenario);

    match run_test(&command_list, &mut *test_tree) {
        Err(e) => {
            log::error!("FAIL when running scenario {:?}", &filename);
            log::error!("{:?}", e);
            result.insert(filename.into(), e.to_string());
        }
        Ok(_) => {
            log::info!("Scenario {:?} SUCCESS", filename);
            result.insert(filename.into(), true.to_string());
        }
    }
    // free leak of the file path
    rust_ffi::free_scenario(command_list);
    let _ = unsafe { CString::from_raw(scenario) };
}

pub fn run_command<'a, T: TestTree + ?Sized>(
    command: &Command,
    tree: &mut T,
) -> Result<()> {
    match command.id {
        CommandId::Insert => {
            let key = command.key();
            let value = command.value();

            let felt = Felt::from_hex(&value)?;
            tree.insert(&key, &felt.to_bytes_be().to_vec())
        }

        CommandId::Remove => {
            let key = command.key();

            tree.remove(&key)
        }

        CommandId::CheckRootHash => {
            let hash = tree.root_hash().unwrap();
            let hash: [u8; 32] = hash[..32].try_into().unwrap();
            let hash = Felt::from_bytes_be(&hash).to_hex_string();
            let ref_root_hash = command.value();

            if &hash != &ref_root_hash {
                Err(anyhow::Error::msg(format!(
                    "Root hash mismatch: {:?} != {:?}",
                    hash, ref_root_hash
                )))
            } else {
                Ok(())
            }
        }

        CommandId::Get => {
            let key = command.key();
            let value = command.value();

            let get = tree
                .get(&key)
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;
            let get: [u8; 32] = get[..32].try_into().unwrap();
            let get = Felt::from_bytes_be(&get);

            if get != Felt::from_hex(&value).unwrap() {
                Err(anyhow::Error::msg(format!(
                    "Value mismatch: {:?} != {:?}",
                    get, value
                )))
            } else {
                Ok(())
            }
        }

        CommandId::Contains => {
            let key = command.key();
            let value = command.value();

            // println!("contains {:?} {}", key, value);

            let res = tree
                .contains(&key)
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            if res != value.parse::<bool>()? {
                Err(anyhow::Error::msg(format!(
                    "Contains mismatch: {:?} != {:?}",
                    res, value
                )))
            } else {
                Ok(())
            }
        }
    }
}

pub fn run_test<T: TestTree + ?Sized>(
    command_list: &rust_ffi::CommandList,
    tree: &mut T,
) -> Result<()> {
    let commands = unsafe {
        std::slice::from_raw_parts(
            command_list.test_commands as *mut Command,
            command_list.len,
        )
    };

    for command in commands {
        run_command(&command, tree)?;
    }

    Ok(())
}
