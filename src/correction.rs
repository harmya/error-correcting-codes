pub enum CorrectionType {
    Parity,
    Triple,
    Hamming,
}

pub fn encode_parity_bit(encoded_string: String) -> (bool, String) {
    let mut count_of_ones = 0;

    for char in encoded_string.chars() {
        if char == '1' {
            count_of_ones += 1;
        }
    }

    if count_of_ones % 2 == 0 {
        return (true, format!("0{}", encoded_string));
    } else {
        return (true, format!("1{}", encoded_string));
    }
}

pub fn decode_parity_bit(encoded_string: &String) -> (bool, bool, String) {
    let mut count_of_ones = 0;

    for char in encoded_string.chars() {
        if char == '1' {
            count_of_ones += 1;
        }
    }

    if count_of_ones % 2 == 0 {
        return (false, false, format!("No error in message"));
    } else {
        return (true, false, format!("STOP! Found error in message"));
    }
}

pub fn encode_triple(encoded_string: String) -> (bool, String) {
    let message_length = encoded_string.len();

    if message_length >= 511 {
        return (
            false,
            "Exceeded the length of allowed message size".to_string(),
        );
    }

    let encoded_string = encoded_string.repeat(3);
    let mut binary_string = format!("{:0>width$b}", message_length, width = 9);
    binary_string.push_str(&encoded_string);
    return (true, binary_string);
}

pub fn decode_triple(encoded_string: &String) -> (bool, bool, String) {
    let first_nine_bits = &encoded_string[..9];
    let length_of_message = usize::from_str_radix(first_nine_bits, 2).unwrap();

    let mut first_chunk: Vec<char> = encoded_string[9..9 + length_of_message].chars().collect();
    let second_chunk: Vec<char> = encoded_string[9 + length_of_message..9 + length_of_message * 2]
        .chars()
        .collect();
    let third_chunk: Vec<char> = encoded_string
        [9 + length_of_message * 2..9 + length_of_message * 3]
        .chars()
        .collect();

    let mut num_errors: usize = 0;
    for i in 0..length_of_message {
        let first_chunk_char = first_chunk[i];
        let second_chunk_char = second_chunk[i];
        let third_chunk_char = third_chunk[i];

        let count_0 = (first_chunk_char == '0') as u32
            + (second_chunk_char == '0') as u32
            + (third_chunk_char == '0') as u32;

        let count_1 = (first_chunk_char == '1') as u32
            + (second_chunk_char == '1') as u32
            + (third_chunk_char == '1') as u32;

        if !(count_0 == 3 || count_1 == 3) {
            num_errors += 1;
        }

        first_chunk[i] = if count_1 >= count_0 { '1' } else { '0' };
    }

    let final_string: String = first_chunk.into_iter().collect();

    return (num_errors > 0, num_errors > 0, final_string);
}

pub fn encode_hamming(encoded_string: String, is_extended: bool) -> (bool, String) {
    let message_length = encoded_string.len();

    if message_length >= 502 {
        return (
            false,
            "Exceeded the length of allowed message size".to_string(),
        );
    }

    let mut num_parity_bits = 0;
    while (1 << num_parity_bits) < (message_length + num_parity_bits + 1) {
        num_parity_bits += 1;
    }

    let total_length = message_length + num_parity_bits;
    let mut hamming_code = vec!['0'; total_length];

    let mut j = 0;
    for i in 1..=total_length {
        if i.is_power_of_two() {
            continue;
        }
        hamming_code[i - 1] = encoded_string.chars().nth(j).unwrap();
        j += 1;
    }

    for i in 0..num_parity_bits {
        let parity_pos = 1 << i;
        let mut parity = 0;

        for bit in (parity_pos..=total_length).step_by(2 * parity_pos) {
            for k in 0..parity_pos {
                if bit + k - 1 < total_length {
                    parity ^= (hamming_code[bit + k - 1] as u8 - b'0') as i32;
                }
            }
        }

        hamming_code[parity_pos - 1] = if parity % 2 == 0 { '0' } else { '1' };
    }

    let final_code: String = hamming_code.into_iter().collect();

    return (true, final_code);
}

pub fn decode_hamming(received_code: &String, is_extended: bool) -> (bool, bool, String) {
    let mut hamming_code: Vec<char> = received_code.chars().collect();
    let mut received_overall_parity: bool = false;
    if is_extended {
        let overall_parity_bit = hamming_code.remove(0);
        received_overall_parity = overall_parity_bit == '0';
    }
    let total_length = hamming_code.len();
    let mut num_parity_bits = 0;
    while (1 << num_parity_bits) < total_length {
        num_parity_bits += 1;
    }

    let mut curr_error_position: usize = 0;
    let mut has_error = false;
    for i in 0..num_parity_bits {
        let parity_pos = 1 << i;
        let mut parity = 0;

        for bit in (parity_pos..=total_length).step_by(2 * parity_pos) {
            println!("bit {}", bit);
            for k in 0..parity_pos {
                if bit + k - 1 <= total_length - 1 {
                    parity ^= (hamming_code[bit + k - 1] as u8 - b'0') as i32;
                }
            }
        }

        println!("Parity found{}", parity);

        if parity % 2 != 0 {
            println!("Parity pos{}", parity_pos);
            curr_error_position += parity_pos;
        }

        println!();
    }
    let mut has_double_error = false;
    println!("{}", curr_error_position);
    if curr_error_position > 0 {
        let idx = curr_error_position - 1;

        if idx < hamming_code.len() {
            has_error = true;
            hamming_code[idx] = if hamming_code[idx] == '0' { '1' } else { '0' };
        }

        let computed_overall_parity = hamming_code.iter().filter(|&&c| c == '1').count() % 2 == 0;
        if is_extended && computed_overall_parity != received_overall_parity {
            has_double_error = true;
        }
    }

    let mut original_data = String::new();
    for i in 1..=total_length {
        if !i.is_power_of_two() {
            original_data.push(hamming_code[i - 1]);
        }
    }

    (has_error, (has_error && !has_double_error), original_data)
}
pub fn encode_correction(
    correction_type: CorrectionType,
    encoded_string: String,
) -> (bool, String) {
    match correction_type {
        CorrectionType::Parity => encode_parity_bit(encoded_string),
        CorrectionType::Triple => encode_triple(encoded_string),
        CorrectionType::Hamming => encode_hamming(encoded_string, false),
    }
}

pub fn decode_correction(
    correction_type: CorrectionType,
    encoded_string: &String,
    is_extended: bool,
) -> (bool, bool, String) {
    match correction_type {
        CorrectionType::Parity => decode_parity_bit(&encoded_string),
        CorrectionType::Triple => decode_triple(&encoded_string),
        CorrectionType::Hamming => decode_hamming(&encoded_string, is_extended),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_parity_bit_even_ones() {
        let input = String::from("1100");
        let expected = (true, String::from("01100"));
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_parity_bit_odd_ones() {
        let input = String::from("1101");
        let expected = (true, String::from("11101"));
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_decode_parity_bit_no_error() {
        let input = String::from("01100");
        let expected = (false, false, String::from("No error in message"));
        assert_eq!(decode_parity_bit(&input), expected);
    }

    #[test]
    fn test_decode_parity_bit_with_error() {
        let input = String::from("111011");
        let expected = (true, false, String::from("STOP! Found error in message"));
        assert_eq!(decode_parity_bit(&input), expected);
    }

    #[test]
    fn test_encode_triple() {
        let input = String::from("111101");
        let expected = (true, String::from("000000110111101111101111101"));
        assert_eq!(encode_triple(input), expected);
    }

    #[test]
    fn test_decode_triple_without_flip() {
        let input = String::from("000000110111101111101111101");
        let expected = (false, false, String::from("111101"));
        assert_eq!(decode_triple(&input), expected);
    }

    #[test]
    fn test_decode_triple_with_one_flip() {
        let input = String::from("000000110111101111101111100");
        let expected = (true, true, String::from("111101"));
        assert_eq!(decode_triple(&input), expected);
    }

    #[test]
    fn test_encode_hamming_one() {
        let input = String::from("11101");
        let expected = (true, String::from("101011011"));
        assert_eq!(encode_correction(CorrectionType::Hamming, input), expected);
    }

    #[test]
    fn test_decode_hamming_one() {
        let input = String::from("101011011");
        let expected = (false, false, String::from("11101"));
        println!("{:?}", expected);
        assert_eq!(
            decode_correction(CorrectionType::Hamming, &input, false),
            expected
        );
    }

    #[test]
    fn test_decode_hamming_with_single_bit_error() {
        let input = String::from("101111011");
        let expected = (true, true, String::from("11101"));
        assert_eq!(
            decode_correction(CorrectionType::Hamming, &input, false),
            expected
        );
    }

    #[test]
    fn test_decode_hamming_with_double_bit_error() {
        let input = String::from("0101111111");
        let expected = (true, false, String::from("01111"));
        assert_eq!(
            decode_correction(CorrectionType::Hamming, &input, true),
            expected
        );
    }
}
