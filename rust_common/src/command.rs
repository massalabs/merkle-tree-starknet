use bitvec::{bitvec, prelude::Msb0, vec::BitVec};
use rust_ffi::{Command, CommandId};

fn extend_key_251(key: BitVec<u8, Msb0>) -> BitVec<u8, Msb0> {
    let mut key251: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0; 251];
    key251.truncate(key251.len() - key.len());

    key251.extend(key.iter());
    key251
}

pub trait CommandTrait {
    fn key(&self) -> BitVec<u8, Msb0>;
    fn value(&self) -> String;
    fn get_arg1(&self) -> String;
}

impl CommandTrait for Command {
    fn value(&self) -> String {
        match self.id {
            CommandId::Insert | CommandId::Contains | CommandId::Get => {
                let arr = unsafe {
                    std::slice::from_raw_parts(
                        self.arg2.ptr as *mut u8,
                        self.arg2.len,
                    )
                };

                // FIME: added null at end of the slice to conform C strings
                // make the len odd, droping it

                let mut arr = arr.to_vec();
                arr.pop();

                String::from_utf8(arr).unwrap()
            }

            CommandId::CheckRootHash => self.get_arg1(),

            _ => unimplemented!("Command has no value"),
        }
    }

    fn get_arg1(&self) -> String {
        let arr = unsafe {
            std::slice::from_raw_parts(self.arg1.ptr as *mut u8, self.arg1.len)
        };

        String::from_utf8(arr.to_vec()).unwrap()
    }

    fn key(&self) -> BitVec<u8, Msb0> {
        match self.id {
            CommandId::Remove
            | CommandId::Insert
            | CommandId::Contains
            | CommandId::Get => {
                let arr = unsafe {
                    std::slice::from_raw_parts(
                        self.arg1.ptr as *mut u8,
                        self.arg1.len,
                    )
                };
                let key = BitVec::from_vec(arr.to_vec());
                let key = extend_key_251(key);
                key
            }

            _ => unimplemented!("Command has no key"),
        }
    }
}
