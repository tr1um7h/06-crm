use crm::pb::User;

// cd to dir crm, then call "cargo run"
pub mod pb {

    include!(concat!(env!("OUT_DIR"), "/crm.rs"));
}

fn main() {
    let user = User::new(1, "Alice", "alice@acme.org");
    // let encoded = user.encode_to_vec();
    // let decoded = User::decode(&encoded[..]).unwrap();
    println!("user: {:?}", user);
}
