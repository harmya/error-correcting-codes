use rand::Rng;

use crate::correction::CorrectionType;

pub fn add_noise(mut message: String, correction_type: CorrectionType) -> String {
    let len_message = message.len();

    let where_to_flip = match correction_type {
        CorrectionType::Parity => rand::thread_rng().gen_range(1..len_message - 1),
        CorrectionType::Triple => todo!(),
        CorrectionType::Hamming => todo!(),
    };

    let mut flipped_bit = &message[where_to_flip..where_to_flip + 1];

    if flipped_bit == "1" {
        flipped_bit = "0";
    } else {
        flipped_bit = "0";
    }

    message.replace_range(where_to_flip..where_to_flip + 1, flipped_bit);

    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_noise_to_parity_correction() {
        let input = String::from("11001010");
        let output = add_noise(input, CorrectionType::Parity);
        let mut count_ones = 0;
        for char in output.chars() {
            if char == '1' {
                count_ones += 1;
            }
        }
        let parity = count_ones % 2;

        assert_eq!(parity, 1);
    }
}
