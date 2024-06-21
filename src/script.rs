use crate::lamport::{create_public_key, hash, random_private_key, sign};
use bitcoin_script::bitcoin_script;

pub fn tap_root(price: u64) {
    let private_key = random_private_key(); //generate a random lamport private key
    let hashed_price = hash(price.to_string().as_str()); //hash the BTC price into 256 bit
    let signature = sign(hashed_price, &private_key);
    let public_key = create_public_key(&private_key);

    let taproot_script = bitcoin_script! {
        OP_IF
            // Signing the fetched BTC price using Lamport private key
            // (Implementation details omitted here for brevity)
            <signature.to_bytes()>
            OP_SHA256 OP_SWAP <public_key.to_bytes()>
        OP_ELSE
            // If the signed price is not less than the stored price
            // (Implementation details omitted here for brevity)
            OP_EQUALVERIFY
            OP_CHECKSIG
        OP_ENDIF
        OP_EQUALVERIFY
        OP_CHECKSIG
    };

    println!("Taproot script generated:\n{}", taproot_script);
}

fn _string_to_bytes(value: String) -> Vec<u8> {
    value.into_bytes()
}

fn _bytes_to_string(value: Vec<u8>) -> String {
    let json_str = String::from_utf8_lossy(&value);

    serde_json::from_str(&json_str).unwrap()
}
