pub fn add_parity_bit(encoded_string: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_parity_bit_even_ones() {
        let input = String::from("1100");
        let expected = String::from("01100");
        assert_eq!(add_parity_bit(input), expected);
    }

    #[test]
    fn test_add_parity_bit_odd_ones() {
        let input = String::from("1101");
        let expected = String::from("11101");
        assert_eq!(add_parity_bit(input), expected);
    }

    #[test]
    fn test_add_parity_bit_no_ones() {
        let input = String::from("0000");
        let expected = String::from("00000");
        assert_eq!(add_parity_bit(input), expected);
    }

    #[test]
    fn test_add_parity_bit_all_ones() {
        let input = String::from("1111");
        let expected = String::from("01111");
        assert_eq!(add_parity_bit(input), expected);
    }
}
