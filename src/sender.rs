use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use huffman::HuffmanEncoding;

pub mod huffman;

const VALID_WORDS: [&str; 12] = [
    "hello", "diya", "how", "are", "you", " ", "#", "mikail", "saad", "sagar", "is", "stupid",
];

fn uses_valid_vocab(message: &str, vocab: &HashSet<&str>) -> bool {
    let words: Vec<&str> = message.split(' ').collect();

    for word in words {
        if !vocab.contains(word) {
            return false;
        }
    }

    return true;
}

fn encode_message(message: &str, hf: &HuffmanEncoding) -> String {
    let mut encoded_message: String = "".to_string();

    for char in message.chars() {
        let bit_string = match hf.encoding.get(&char) {
            Some(val) => val,
            None => panic!("Could not find bit string for char {}", char),
        };
        encoded_message.push_str(&bit_string);
    }

    return encoded_message;
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    println!("Connected to the receiver!");

    let vocab = HashSet::from(VALID_WORDS);
    let hf = HuffmanEncoding::new(&VALID_WORDS);
    let table_encoding = HuffmanEncoding::encode_table(&hf);
    stream.write_all(table_encoding.as_bytes()).unwrap();

    loop {
        println!("Enter a message to send (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if !uses_valid_vocab(input, &vocab) {
            println!("Please only use words in the vocabulary: {:?}", VALID_WORDS);
            continue;
        }

        let input = format!("{}#", input);

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        let encoded_message = encode_message(&input, &hf);
        println!("Encoded Message: {}", encoded_message);

        stream.write_all(encoded_message.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("From Receiver: {}", response);
                }
            }
            Err(e) => {
                println!("Failed to read response: {}", e);
            }
        }
    }

    Ok(())
}
