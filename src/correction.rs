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

pub fn decode_parity_bit(encoded_string: String) -> (bool, String) {
    let mut count_of_ones = 0;

    for char in encoded_string.chars() {
        if char == '1' {
            count_of_ones += 1;
        }
    }

    if count_of_ones % 2 == 0 {
        return (true, format!("No error in message"));
    } else {
        return (false, format!("STOP! Found error in message"));
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

pub fn decode_triple(encoded_string: String) -> (bool, String) {
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

        first_chunk[i] = if count_1 >= count_0 { '1' } else { '0' };
    }

    let final_string: String = first_chunk.into_iter().collect();

    return (true, final_string);
}

pub fn encode_hamming(encoded_string: String) -> (bool, String) {
    let zero_index: String = "11111110".to_string();

    let message_length = encoded_string.len();

    if message_length >= 502 {
        return (
            false,
            "Exceeded the length of allowed message size".to_string(),
        );
    }

    let num_parity_bits = message_length.ilog2() + 1;
    println!("Number of parity bits {}", num_parity_bits);
    return (true, "".to_string());
}
pub fn encode_correction(
    correction_type: CorrectionType,
    encoded_string: String,
) -> (bool, String) {
    match correction_type {
        CorrectionType::Parity => encode_parity_bit(encoded_string),
        CorrectionType::Triple => encode_triple(encoded_string),
        CorrectionType::Hamming => encode_hamming(encoded_string),
    }
}

pub fn decode_correction(
    correction_type: CorrectionType,
    encoded_string: String,
) -> (bool, String) {
    match correction_type {
        CorrectionType::Parity => decode_parity_bit(encoded_string),
        CorrectionType::Triple => decode_triple(encoded_string),
        CorrectionType::Hamming => todo!(),
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
    fn test_encode_parity_bit_no_ones() {
        let input = String::from("0000");
        let expected = (true, String::from("00000"));
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_parity_bit_all_ones() {
        let input = String::from("1111");
        let expected = (true, String::from("01111"));
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_decode_parity_bit_no_error() {
        let input = String::from("01100");
        let expected = (true, String::from("No error in message"));
        assert_eq!(decode_parity_bit(input), expected);
    }

    #[test]
    fn test_decode_parity_bit_with_error() {
        let input = String::from("111011");
        let expected = (false, String::from("STOP! Found error in message"));
        assert_eq!(decode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_correction_parity() {
        let input = String::from("1101");
        let expected = (true, String::from("11101"));
        assert_eq!(encode_correction(CorrectionType::Parity, input), expected);
    }

    #[test]
    fn test_decode_correction_parity_no_error() {
        let input = String::from("01100");
        let expected = (true, String::from("No error in message"));
        assert_eq!(decode_correction(CorrectionType::Parity, input), expected);
    }

    #[test]
    fn test_decode_correction_parity_with_error() {
        let input = String::from("111101");
        let expected = (false, String::from("STOP! Found error in message"));
        assert_eq!(decode_correction(CorrectionType::Parity, input), expected);
    }

    #[test]
    fn test_encode_triple() {
        let input = String::from("111101");
        let expected = (true, String::from("000000110111101111101111101"));
        assert_eq!(encode_triple(input), expected);
    }

    #[test]
    fn test_encode_correction_with_triple() {
        let input = String::from("111101");
        let expected = (true, String::from("000000110111101111101111101"));
        assert_eq!(encode_correction(CorrectionType::Triple, input), expected);
    }

    #[test]
    fn test_decode_triple_without_flip() {
        let input = String::from("000000110111101111101111101");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_triple(input), expected);
    }

    #[test]
    fn test_decode_triple_with_one_flip() {
        let input = String::from("000000110111101111101111100");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_triple(input), expected);
    }

    #[test]
    fn test_decode_triple_with_two_flip() {
        let input = String::from("000000110101101111111111101");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_triple(input), expected);
    }

    #[test]
    fn test_decode_correction_with_triple_without_flip() {
        let input = String::from("000000110111101111101111101");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_correction(CorrectionType::Triple, input), expected);
    }

    #[test]
    fn test_decode_correction_with_triple_with_one_flip() {
        let input = String::from("000000110111101111101111100");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_correction(CorrectionType::Triple, input), expected);
    }

    #[test]
    fn test_decode_correction_with_triple_with_two_flip() {
        let input = String::from("000000110111101111101111110");
        let expected = (true, String::from("111101"));
        assert_eq!(decode_correction(CorrectionType::Triple, input), expected);
    }

    #[test]
    fn test_encode_hamming() {
        let input = String::from("000000110111101111101111110");
        let expected = (true, String::from(""));
        assert_eq!(encode_correction(CorrectionType::Hamming, input), expected);
    }
}
