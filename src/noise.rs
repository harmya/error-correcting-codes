use rand::Rng;

use crate::correction::CorrectionType;

pub fn add_noise(message: String, correction_type: &CorrectionType) -> String {
    let len_message = message.len();
    let mut rng = rand::thread_rng();
    let mut output = message.clone();

    match correction_type {
        CorrectionType::Parity => {
            let where_to_flip = rng.gen_range(10..len_message - 1);
            flip_bit(&mut output, where_to_flip);
        }
        CorrectionType::Triple => {
            if len_message < 10 {
                return output; // Make sure message length is sufficient
            }

            let x = rng.gen_range(1..=(len_message / 3));
            let first_chunk_start = 11;
            let first_chunk_end = len_message / 3;
            let num_bits_to_flip = x / 2;

            let mut flipped_positions = vec![];
            while flipped_positions.len() < num_bits_to_flip {
                let pos = rng.gen_range(first_chunk_start..first_chunk_end);
                if !flipped_positions.contains(&pos) {
                    flip_bit(&mut output, pos);
                    flipped_positions.push(pos);
                }
            }
        }
        CorrectionType::Hamming => {
            let num_bits_to_flip = rng.gen_range(1..=2); // Randomly choose 1 or 2 bits to flip
            let mut flipped_positions = vec![];

            while flipped_positions.len() < num_bits_to_flip {
                let pos = rng.gen_range(1..len_message - 1);
                if !flipped_positions.contains(&pos) {
                    flip_bit(&mut output, pos);
                    flipped_positions.push(pos);
                }
            }
        }
    }

    output
}

fn flip_bit(message: &mut String, index: usize) {
    if let Some(bit) = message.chars().nth(index) {
        let flipped_bit = if bit == '1' { '0' } else { '1' };
        message.replace_range(index..index + 1, &flipped_bit.to_string());
    }
}
