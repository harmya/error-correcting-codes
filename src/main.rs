use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", received_message);

        let response = b"Message received!";
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

/*

let hf = HuffmanEncoding::new(&VOCAB);
println!("Encoding Table: {:?}", hf);

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
*/
