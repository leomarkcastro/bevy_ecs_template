use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSaveData {
    pub player_health: i32,
}

const SECRET_SAVE_KEY: &str = "HEAVEN ARE ONLY FOR THOSE WHO SUFFER";

pub fn serialize_encrypt(data: &GlobalSaveData) -> String {
    let mc = new_magic_crypt!(SECRET_SAVE_KEY, 256);
    let data_string = serde_json::to_string(data).unwrap();
    mc.encrypt_str_to_base64(data_string)
}

pub fn deserialize_decrypt(data: &str) -> GlobalSaveData {
    let mc = new_magic_crypt!(SECRET_SAVE_KEY, 256);
    let data_string = mc.decrypt_base64_to_string(data).unwrap();
    serde_json::from_str(&data_string).unwrap()
}
