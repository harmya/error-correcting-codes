### Implementation of algorithms for error correction codes

Use huffman encoding to compress and decompress data. Implement error correcting codes for this encoded data and verify by randomly flipping bits in the encoded data.

### Updates

Can send the huffman encoding table as a part of the message:

**Table to encode**: HuffmanEncoding { encoding: {'j': "00000", 'e': "01", 'f': "1001", 'a': "110", 'i': "1110", 'c': "00001", 'k': "1111", 'b': "0001", 'l': "101", 'h': "1000", 'd': "001"}, max_size: 5 }

**Message that is sent**: "000001010011010000000100010010010110010010000001111001000100111000010101000010101010011110000110000010101101110100111100100000011011001"

**Table that is decoded**: HuffmanEncoding { encoding: {'i': "1110", 'e': "01", 'b': "0001", 'l': "101", 'd': "001", 'f': "1001", 'a': "110", 'h': "1000", 'c': "00001", 'k': "1111", 'j': "00000"}, max_size: 5 }
