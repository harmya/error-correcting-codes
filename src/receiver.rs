use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use huffman::{HuffmanDecoding, HuffmanEncoding};

pub mod huffman;

fn decode_message(message: &str, hd: &HuffmanDecoding) -> String {
    if hd.max_size == 0 {
        return "".to_string();
    }

    let mut decoded_string: String = "Your message is ".to_string();

    let mut left_ptr = 0;
    let mut right_ptr = 1;

    while right_ptr < message.len() && left_ptr < message.len() {
        let string_slice = &message[left_ptr..right_ptr];
        match hd.decoding.get(string_slice) {
            Some(value) => {
                decoded_string.push(*value);
                left_ptr = right_ptr;
                right_ptr += 1;
            }
            None => right_ptr += 1,
        }
    }

    return decoded_string;
}
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let mut message_count = 0;

    let mut decoding_table: HuffmanDecoding = HuffmanDecoding {
        decoding: HashMap::new(),
        max_size: 0,
    };

    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", received_message);

        if message_count == 0 {
            decoding_table = HuffmanEncoding::decode_table(&received_message).unwrap();
            message_count += 1;
            continue;
        }

        let mut decoded_message = decode_message(&received_message, &decoding_table);

        if decoded_message == "" {
            decoded_message = "Encoding Table not sent :(".to_string();
        }

        let response = decoded_message.as_bytes();
        stream.write_all(response).expect("Failed to send response");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    println!("Receiver is listening on 127.0.0.1:6969");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");
                handle_client(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
