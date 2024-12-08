pub enum CorrectionType {
    Parity,
    Triple,
    Hamming,
}

pub fn encode_parity_bit(encoded_string: String) -> String {
    let mut count_of_ones = 0;

    for char in encoded_string.chars() {
        if char == '1' {
            count_of_ones += 1;
        }
    }

    if count_of_ones % 2 == 0 {
        return format!("0{}", encoded_string);
    } else {
        return format!("1{}", encoded_string);
    }
}

pub fn decode_parity_bit(encoded_string: String) -> String {
    let mut count_of_ones = 0;

    for char in encoded_string.chars() {
        if char == '1' {
            count_of_ones += 1;
        }
    }

    if count_of_ones % 2 == 0 {
        return format!("No error in message");
    } else {
        return format!("STOP! Found error in message");
    }
}

pub fn encode_correction(correction_type: CorrectionType, encoded_string: String) -> String {
    match correction_type {
        CorrectionType::Parity => encode_parity_bit(encoded_string),
        CorrectionType::Triple => todo!(),
        CorrectionType::Hamming => todo!(),
    }
}

pub fn decode_correction(correction_type: CorrectionType, encoded_string: String) -> String {
    match correction_type {
        CorrectionType::Parity => decode_parity_bit(encoded_string),
        CorrectionType::Triple => todo!(),
        CorrectionType::Hamming => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_parity_bit_even_ones() {
        let input = String::from("1100");
        let expected = String::from("01100");
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_parity_bit_odd_ones() {
        let input = String::from("1101");
        let expected = String::from("11101");
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_parity_bit_no_ones() {
        let input = String::from("0000");
        let expected = String::from("00000");
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_parity_bit_all_ones() {
        let input = String::from("1111");
        let expected = String::from("01111");
        assert_eq!(encode_parity_bit(input), expected);
    }

    #[test]
    fn test_decode_parity_bit_no_error() {
        let input = String::from("01100");
        let expected = String::from("No error in message");
        assert_eq!(decode_parity_bit(input), expected);
    }

    #[test]
    fn test_decode_parity_bit_with_error() {
        let input = String::from("111011");
        let expected = String::from("STOP! Found error in message");
        assert_eq!(decode_parity_bit(input), expected);
    }

    #[test]
    fn test_encode_correction_parity() {
        let input = String::from("1101");
        let expected = String::from("11101");
        assert_eq!(encode_correction(CorrectionType::Parity, input), expected);
    }

    #[test]
    fn test_decode_correction_parity_no_error() {
        let input = String::from("01100");
        let expected = String::from("No error in message");
        assert_eq!(decode_correction(CorrectionType::Parity, input), expected);
    }

    #[test]
    fn test_decode_correction_parity_with_error() {
        let input = String::from("111101");
        let expected = String::from("STOP! Found error in message");
        assert_eq!(decode_correction(CorrectionType::Parity, input), expected);
    }
}
