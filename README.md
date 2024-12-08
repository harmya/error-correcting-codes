### Implementation of algorithms for error correction codes

Use huffman encoding to compress and decompress data. Implement error correcting codes for this encoded data and verify by randomly flipping bits in the encoded data. I am still working on implementing the error correction. Will implement Parity, Triple modular redundancy, Hamming Codes, Extended Hamming Codes, RSC

### Updates

Imagine you some vocabulary declared like this:

```rust
const VOCAB: [&str; 12] = [
    "hello", "diya", "how", "are", "you", "mikail", "saad", "sagar", "is", "stupid", " ", "#",
];
```

You can encode a message like this:

```rust
let message = "hello diya how are you";
```

We can use the vocabulary to design a huffman encoding table that we can send as a part of the message:

```rust
HuffmanEncoding { encoding: {'e': "0000", 'u': "0011", 'a': "110", 'g': "111010", 'p': "111101", 't': "111110", 'w': "111111", 'o': "1011", 'd': "0111", 'i': "100", 'k': "111011", 'h': "0001", 'r': "0010", 's': "010", 'y': "0110", 'l': "1010", ' ': "111000", 'm': "111100", '#': "111001"}, max_size: 6 }
```

Now, using this we can encode the message as:

```rust
Encoded Message: 111100100111011110100101011100010001011100001011111000111111011000111111001
```

After receving the message and the table, we first decode the table, then we decode the message using the table.

Now implementing parity bit error correction
