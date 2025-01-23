mod verify;
mod command_line;

use clap::Parser;

fn main() {

    let args = command_line::Args::parse();

    let pub_key_file = args.key;
    let msg_file = args.message;

    let public_key = verify::signed_key(pub_key_file);

    let msg = verify::message(msg_file);

    msg.verify(&public_key).unwrap();

    let msg_content = msg.get_content().unwrap();
    let msg_string = String::from_utf8(msg_content.unwrap()).expect("expect UTF8");
    println!("Signed message:\n{:?}", msg_string);
}
