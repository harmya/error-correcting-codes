use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type HuffmanNodeRef = Option<Box<HuffmanNode>>;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct HuffmanNode {
    pub frequency: usize,
    pub is_leaf: bool,
    pub character: Option<char>,
    pub left_node: HuffmanNodeRef,
    pub right_node: HuffmanNodeRef,
}

impl HuffmanNode {
    pub fn save_encoding(root: &HuffmanNodeRef, curr_code: String) -> HuffmanEncoding {
        let mut map = HuffmanEncoding {
            encoding: HashMap::new(),
            max_size: 0,
        };
        Self::save_encoding_helper(root, curr_code, &mut map);
        return map;
    }

    fn save_encoding_helper(root: &HuffmanNodeRef, curr_code: String, map: &mut HuffmanEncoding) {
        if let Some(node) = root {
            if node.is_leaf {
                if let Some(c) = node.character {
                    map.max_size = cmp::max(map.max_size, curr_code.len());
                    map.encoding.insert(c, curr_code);
                }
            } else {
                Self::save_encoding_helper(&node.left_node, format!("{}0", curr_code), map);
                Self::save_encoding_helper(&node.right_node, format!("{}1", curr_code), map);
            }
        }
    }
}

#[derive(Debug)]
pub struct HuffmanEncoding {
    pub encoding: HashMap<char, String>,
    pub max_size: usize,
}

impl HuffmanEncoding {
    pub fn new(words: &[&str]) -> Self {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for &word in words {
            for char in word.chars() {
                let _ = match counts.get(&char) {
                    Some(value) => counts.insert(char, value + 1),
                    None => counts.insert(char, 1),
                };
            }
        }

        let mut min_heap: BinaryHeap<Reverse<HuffmanNode>> = BinaryHeap::new();

        for (k, v) in counts {
            min_heap.push(Reverse(HuffmanNode {
                frequency: v,
                is_leaf: true,
                character: Option::Some(k),
                left_node: None,
                right_node: None,
            }));
        }

        while min_heap.len() > 1 {
            if let (Some(first), Some(second)) = (min_heap.pop(), min_heap.pop()) {
                let new_freq = first.0.frequency + second.0.frequency;
                let new_internal: HuffmanNode = HuffmanNode {
                    frequency: new_freq,
                    is_leaf: false,
                    character: None,
                    left_node: Some(Box::new(first.0)),
                    right_node: Some(Box::new(second.0)),
                };
                min_heap.push(Reverse(new_internal));
            } else {
                println!("Heap does not contain enough elements.");
            }
        }

        let curr_root = Box::new(min_heap.pop().unwrap().0);
        let map = HuffmanNode::save_encoding(&Some(curr_root), "".to_string());
        return map;
    }

    pub fn encode_table(hf: HuffmanEncoding) -> String {
        /* this sends data in chunks of 5 + max_size.
         * Count all the zeros until I hit a 1. That the my max_size.
         * After that 1, read in chunks of 5 + max_size to get the alphabet and its encoding
         * the first 5 bits tell us what letter of the alphabet
         * and the next 5 tell us the huffman code for it
         */

        let mut to_send = String::from(format!("{}1", "0".repeat(hf.max_size)));
        let bits_req = (hf.max_size.ilog2() + 1) as usize;

        for (k, v) in hf.encoding {
            let num_alphabet = (k as usize) - 97;
            let binary_string = format!("{:0>width$b}", num_alphabet, width = 5);
            to_send.push_str(&binary_string);

            let size_of_code = v.len();
            let binary_size_of_code = format!("{:0>width$b}", size_of_code, width = bits_req);
            to_send.push_str(&binary_size_of_code);
            to_send.push_str(&v);
        }

        return to_send;
    }

    pub fn decode_table(s: &str) -> Option<HuffmanEncoding> {
        if s.is_empty() {
            return None;
        }

        let mut hf = HuffmanEncoding {
            encoding: HashMap::new(),
            max_size: 0,
        };

        let mut max_size: usize = 0;

        let mut chars = s.chars();

        while let Some('0') = chars.next() {
            max_size += 1;
        }

        hf.max_size = max_size;

        let mut index = max_size + 1;
        let bits_req = (max_size.ilog2() + 1) as usize;

        while index <= s.len() {
            // Read the first 5 bits to get what alphaber number it is
            let num_bits = &s[index..index + 5];
            let number = usize::from_str_radix(num_bits, 2).unwrap();
            index += 5;

            // Read the next bits_req bits to see the length of the huffman encoding
            let length_bits = &s[index..index + bits_req];
            let bits_to_read = usize::from_str_radix(length_bits, 2).unwrap();
            index += bits_req;

            // Read the number of bits specified by bits_to_read to get the encoding
            let str_bits = &s[index..index + bits_to_read];
            let string = str_bits.chars().collect::<String>();
            index += bits_to_read;

            hf.encoding.insert((number as u8 + b'a') as char, string);

            // Ensure you don't go out of bounds
            if index >= s.len() {
                break;
            }
        }
        return Some(hf);
    }
}
