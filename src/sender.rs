use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use correction::{encode_correction, CorrectionType};
use huffman::HuffmanEncoding;
use noise::add_noise;

pub mod correction;
pub mod huffman;
pub mod noise;

const VALID_WORDS: [&str; 13] = [
    "hello", "how", "are", "you", " ", "#", "mikail", "saad", "sagar", "is", "sarthak", "so",
    "cooked",
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

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        let correction_type: CorrectionType;
        let type_to_append: char;

        loop {
            println!("Choose error correction method:");
            println!("1. Parity (Detects errors, no correction)");
            println!("2. TPC (Corrects small errors, uses more space)");
            println!("3. Hamming (Detects and corrects single-bit errors)");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");

            match choice.trim() {
                "1" => {
                    correction_type = CorrectionType::Parity;
                    type_to_append = 'P';
                    break;
                }
                "2" => {
                    correction_type = CorrectionType::Triple;
                    type_to_append = 'T';
                    break;
                }
                "3" => {
                    correction_type = CorrectionType::Hamming;
                    type_to_append = 'H';
                    break;
                }
                _ => println!("Invalid choice. Please enter 1, 2, or 3."),
            }
        }

        let encoded_message = encode_message(&input, &hf);
        println!("Encoded Message: {}", encoded_message);

        let error_encoded_message = encode_correction(&correction_type, &encoded_message);

        println!(
            "Error Resistant Encoded Message: {}",
            error_encoded_message.1
        );

        let mut add_noise_to_message = add_noise(error_encoded_message.1, &correction_type);
        add_noise_to_message.push(type_to_append);
        println!("Adding noise to the message, flipping a random bit");

        if error_encoded_message.0 {
            stream.write_all(add_noise_to_message.as_bytes()).unwrap();
        } else {
            println!("Error in encoding the message with error correcting codes")
        }

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
