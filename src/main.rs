use huffman::HuffmanEncoding;

pub mod huffman;

const VOCAB: [&str; 12] = [
    "hello", "diya", "how", "are", "you", " ", "#", "mikail", "saad", "sagar", "is", "stupid",
];

const MESSAGE: &str = "mikail is stupid#";

fn main() {
    let hf = HuffmanEncoding::new(&VOCAB);
    let mut encoded_message: String = "".to_string();

    for char in MESSAGE.chars() {
        let bit_string = match hf.encoding.get(&char) {
            Some(val) => val,
            None => panic!("Could not find bit string for char {}", char),
        };
        encoded_message.push_str(&bit_string);
    }

    println!("Encoded Message: {}", encoded_message);

    let message = HuffmanEncoding::encode_table(hf);
    let decode = HuffmanEncoding::decode_table(&message).unwrap();

    let mut decoded_string: String = "".to_string();

    let mut left_ptr = 0;
    let mut right_ptr = 1;

    while right_ptr < encoded_message.len() && left_ptr < encoded_message.len() {
        let string_slice = &encoded_message[left_ptr..right_ptr];
        match decode.decoding.get(string_slice) {
            Some(value) => {
                decoded_string.push(*value);
                left_ptr = right_ptr;
                right_ptr += 1;
            }
            None => right_ptr += 1,
        }
    }
    println!("Decoded string: {}", decoded_string);
}
