use rand::Rng;

use crate::correction::CorrectionType;

pub fn add_noise(mut message: String, correction_type: CorrectionType) -> String {
    let len_message = message.len();

    let where_to_flip = match correction_type {
        CorrectionType::Parity => rand::thread_rng().gen_range(1..len_message - 1),
        CorrectionType::Triple => todo!(),
        CorrectionType::Hamming => todo!(),
    };

    let mut output = message.clone();
    let mut flipped_bit = &output[where_to_flip..where_to_flip + 1];

    if flipped_bit == "1" {
        flipped_bit = "0";
    } else {
        flipped_bit = "0";
    }

    output.replace_range(where_to_flip..where_to_flip + 1, flipped_bit);

    output
}
