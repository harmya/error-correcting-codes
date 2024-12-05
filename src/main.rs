use huffman::HuffmanEncoding;

pub mod huffman;

const VOCAB: [&str; 30] = [
    "bake", "fade", "cake", "dial", "file", "bead", "heal", "leaf", "held", "jade", "face", "back",
    "beak", "bike", "flake", "head", "hike", "like", "dale", "lake", "deal", "lied", "idle",
    "acid", "chide", "flail", "kale", "behalf", "flake", "life",
];

fn main() {
    let hf = HuffmanEncoding::new(&VOCAB);
    println!("Table to encode: {:?}", hf);

    let message = HuffmanEncoding::encode_table(hf);
    println!("Message that is sent: {:?}", message);

    let decode = HuffmanEncoding::decode_table(&message).unwrap();
    println!("Table that is decoded: {:?}", decode);
}
