fn main() {
    use pgp::{Deserializable, Message, SignedPublicKey};
    use std::fs;

    let pub_key_file = "key.asc";
    let msg_file = "msg.asc";

    let key_string = fs::read_to_string(pub_key_file).unwrap();
    let (public_key, _headers_public) = SignedPublicKey::from_string(&key_string).unwrap();

    let msg_string = fs::read_to_string(msg_file).unwrap();
    let (msg, _headers_msg) = Message::from_string(&msg_string).unwrap();

    // Verify this message
    // NOTE: This assumes that the primary serves as the signing key, which is not always the case!
    msg.verify(&public_key).unwrap();

    let msg_content = msg.get_content().unwrap(); // actual message content
    let msg_string = String::from_utf8(msg_content.unwrap()).expect("expect UTF8");
    println!("Signed message: {:?}", msg_string);
}
