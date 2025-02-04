### Implementation of a sender and receiver that use algorithms for error correction codes to pass messages in a noisy channel

Use huffman encoding to compress and decompress data. Implement error correcting codes for this encoded data and verify by randomly flipping bits in the encoded data. I am still working on implementing the error correction. Will implement Parity, Triple modular redundancy, Hamming Codes, Extended Hamming Codes, RSC

### Setup

Imagine you some vocabulary declared like this:

```rust
const VALID_WORDS: [&str; 13] = [
    "hello", "how", "are", "you", " ", "#", "mikail", "saad", "sagar", "is", "sarthak", "so",
    "cooked",
];
```

You can encode a message like this:

```rust
let message = "hello how are you";
```

We can use the vocabulary to design a huffman encoding table that we can send as a part of the message:

```rust
HuffmanEncoding { encoding: {'e': "0000", 'u': "0011", 'a': "110", 'g': "111010", 'p': "111101", 't': "111110", 'w': "111111", 'o': "1011", 'd': "0111", 'i': "100", 'k': "111011", 'h': "0001", 'r': "0010", 's': "010", 'y': "0110", 'l': "1010", ' ': "111000", 'm': "111100", '#': "111001"}, max_size: 6 }
```

Now, using this we can encode the message as:

```rust
Encoded Message: 111100100111011110100101011100010001011100001011111000111111011000111111001
```

Then, we can select what error correction strategy we want to use:
```
Choose error correction method:
1. Parity (Detects errors, no correction)
2. TPC (Corrects small errors, uses more space)
3. Hamming (Detects and corrects single-bit errors)
```
Now, based on the selected strategy, we add noise to the data in the follwing way:
1. Parity: Flip a random bit
2. TPC: Select the first chunk. Generate a random number k between 1 and length / 2. Randomly select and flip k bits.
3. Hamming: Select a number between 1 and 2. Flip those number of bits.

Now the sender sends this to the receiver (server)

After receving the message and the table, we first decode the table, then we decode the message using the table in the receiver. (We send the table ONCE at the start when the server receives a connection)

### Parity Bit
Most naive method. Can only help in knowing IF an error occurred not WHERE it occurred.
During encoding: just add a 0 or 1 at the start of the message to make sure that the number of ones in the message is even
During decoding: check if number of ones is even, if not then error, otherwise good

### Triple modular redundancy
Still kinda naive. We just repeat the message 3 times.
During encoding: repeat the message three times
During decoding: divide the message in chunks of three, check if each bit matches across all three. If not, then vote 2/3 for the value of that bit.

### Hamming Code
For ocating 1 bit errors. By just using 9 extra bits for a message of length ~500, we can detect and correct 1 bit errors. 
During encoding: Construct an empty message size of length m + parity bits p such that 2^p >= p + m. Now, let the parity bits be p1, p2, p3...Then, p1 makes sure that the parity of every bit location which has 1 in the 1st place (least significant) is even, p2 makes sure that the parity of every bit location which has 1 in the 2nd place (least significant) is even and so on. 
During decoding: Re-check the parity bits similar to the encoding. Keep track of the how many parity bits show error and then add them. Why add? Consider p1 bit is wrong, then I know that some bit with a 1 in the 1st place is wrong, and then if p2 is wrong, I also know that some bit with 1 in the 2nd place is wrong. Hence, the incorrect bit should x...xx11. 

### Extended Hamming Code
Same as hamming code with a difference: can detect but not correct double bit errors. 
During encoding: We use a 0th parity bit to store the parity of the entire message.
During decoding: We first corect the 1 bit errors. Now, if the 0th bit's parity is still wrong, then there is a double bit error.

### Reed Soloman Codes
Todo
