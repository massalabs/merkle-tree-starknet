use anyhow::{anyhow, Result};

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{collections::HashMap, str::FromStr};

use crate::{Bytes, Command, CommandId, CommandList};

#[derive(Debug, Serialize, Deserialize)]
pub struct Insert {
    pub key: Vec<u8>,
    pub value: String,
}

impl From<Insert> for Command {
    fn from(cmd: Insert) -> Self {
        Command::new(
            CommandId::Insert,
            Bytes::new(cmd.key),
            Bytes::from(cmd.value.as_str()),
        )
    }
}

impl Into<String> for Insert {
    fn into(self) -> String {
        format!(
            "- insert:\n  key: {:?}\n  value: {}\n",
            self.key, self.value
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    pub key: Vec<u8>,
    pub expected_value: String,
}

impl From<Get> for Command {
    fn from(cmd: Get) -> Self {
        Command::new(
            CommandId::Get,
            Bytes::new(cmd.key),
            Bytes::from(cmd.expected_value.as_str()),
        )
    }
}

impl Into<String> for Get {
    fn into(self) -> String {
        format!(
            "- get:\n  key: {:?}\n  expected_value: {}\n",
            self.key, self.expected_value
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contains {
    pub key: Vec<u8>,
    pub expected_value: bool,
}

impl From<Contains> for Command {
    fn from(value: Contains) -> Self {
        Command::new(
            CommandId::Contains,
            Bytes::new(value.key),
            Bytes::from(value.expected_value.to_string().as_str()),
        )
    }
}

impl Into<String> for Contains {
    fn into(self) -> String {
        format!(
            "- contains:\n  key: {:?}\n  expected_value: {}\n",
            self.key, self.expected_value
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Remove {
    pub key: Vec<u8>,
}

impl From<Remove> for Command {
    fn from(value: Remove) -> Self {
        Command::new(CommandId::Remove, Bytes::new(value.key), "".into())
    }
}

impl Into<String> for Remove {
    fn into(self) -> String {
        format!("- remove:\n  key: {:?}\n", self.key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckRootHash {
    pub expected_value: String,
}

impl From<CheckRootHash> for Command {
    fn from(value: CheckRootHash) -> Self {
        Command::new(
            CommandId::CheckRootHash,
            Bytes::new(value.expected_value.as_bytes().to_vec()),
            "".into(),
        )
    }
}

impl Into<String> for CheckRootHash {
    fn into(self) -> String {
        format!(
            "- check_root_hash:\n  expected_value: {}\n",
            self.expected_value
        )
    }
}

// Using an enum to represent the different types of commands would have been
// more idiomatic. But serde_yaml parse them as custom tags and custom tags
// are not supported yaml schemas and so editors like VSCode will emit syntax
// errors
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct YCommand {
    #[serde(flatten)]
    all: HashMap<String, serde_yaml::Value>,
}
impl From<YCommand> for Command {
    fn from(value: YCommand) -> Self {
        assert!(value.all.len() == 1);
        let (key, value) = value.all.into_iter().next().unwrap();

        match CommandId::from_str(key.as_str())
            .expect(&format!("Invalid command: {}", key))
        {
            CommandId::Insert => {
                serde_yaml::from_value::<Insert>(value.to_owned())
                    .unwrap()
                    .into()
            }
            CommandId::Remove => {
                serde_yaml::from_value::<Remove>(value.to_owned())
                    .unwrap()
                    .into()
            }
            CommandId::CheckRootHash => {
                serde_yaml::from_value::<CheckRootHash>(value.to_owned())
                    .unwrap()
                    .into()
            }
            CommandId::Get => serde_yaml::from_value::<Get>(value.to_owned())
                .unwrap()
                .into(),
            CommandId::Contains => {
                serde_yaml::from_value::<Contains>(value.to_owned())
                    .unwrap()
                    .into()
            }
        }
    }
}

pub fn parse_yaml(content: &str) -> Result<CommandList> {
    Ok(serde_yaml::from_str::<Vec<YCommand>>(content)
        .map_err(|e| anyhow!(e))?
        // .unwrap()
        .into_iter()
        .map(|cmd| cmd.into())
        .collect::<Vec<Command>>()
        .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml() {
        let scn = r#"
- insert:
    key: [0x01, 0x02, 0x03]
    value: "1"
- get:
    key: [0x01, 0x02, 0x03]
    expected_value: "1"
- contains:
    key: [0x04, 0x05, 0x06]
    expected_value: false
- remove:
    key: [0x01, 0x02, 0x03]
- check_root_hash:
    expected_value: "0xdeadbeef"
"#;
        let commands = vec![
            Command::new(
                CommandId::Insert,
                Bytes::new(vec![0x01, 0x02, 0x03]),
                "1".into(),
            ),
            Command::new(
                CommandId::Get,
                Bytes::new(vec![0x01, 0x02, 0x03]),
                "1".into(),
            ),
            Command::new(
                CommandId::Contains,
                Bytes::new(vec![0x04, 0x05, 0x06]),
                "false".into(),
            ),
            Command::new(
                CommandId::Remove,
                Bytes::new(vec![0x01, 0x02, 0x03]),
                "".into(),
            ),
            Command::new(
                CommandId::CheckRootHash,
                Bytes::new("0xdeadbeef".as_bytes().to_vec()),
                "".into(),
            ),
        ];

        let cmd_list = parse_yaml(scn).unwrap();
        dbg!(&cmd_list);
        dbg!(&cmd_list.test_commands);

        // verify that cmd_list contains the same commands as commands
        assert_eq!(cmd_list.len, commands.len());

        for i in 0..cmd_list.len {
            let cmd = unsafe { cmd_list.test_commands.add(i).read() };
            assert_eq!(cmd.id, commands[i].id);
            assert_eq!(cmd.arg1.len, commands[i].arg1.len);
            assert_eq!(cmd.arg2.len, commands[i].arg2.len);
        }
    }

    #[test]
    fn test_parse_small_yaml() {
        let scn = r#"
        - insert:
            key: [1]
            value: "0x1"
        "#;
        let commands = vec![Command::new(
            CommandId::Insert,
            Bytes::new(vec![1]),
            "0x1".into(),
        )];
        let cmd_list = parse_yaml(scn).unwrap();
        assert_eq!(cmd_list.len, commands.len());
        for i in 0..cmd_list.len {
            let cmd = unsafe { cmd_list.test_commands.add(i).read() };
            assert_eq!(cmd.id, commands[i].id);
            assert_eq!(cmd.arg1.len, commands[i].arg1.len);
            assert_eq!(cmd.arg2.len, commands[i].arg2.len);
        }
    }
}
