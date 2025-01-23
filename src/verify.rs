use pgp::{Deserializable, Message, SignedPublicKey};
use std::fs;

pub fn signed_key(pub_key_file: String) -> SignedPublicKey {
    let key_string = match fs::read_to_string(pub_key_file) {
        Ok(key) => key,
        Err(error) =>panic!("Problem opening key: {error}."),
    };

    let (public_key, _headers_public) = SignedPublicKey::from_string(&key_string).unwrap();
    public_key
}

pub fn message(msg_file: String) -> Message {
    let msg_string = match fs::read_to_string(msg_file) {
        Ok(file) => file,
        Err(error) => panic!("Proglem opening message: {error}."),
    };
    let (msg, _headers_msg) = Message::from_string(&msg_string).unwrap();
    msg
}
