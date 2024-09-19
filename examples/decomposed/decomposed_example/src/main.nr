global table = comptime { make_lookup_table() };
pub fn regex_match<let N: u32>(input: [u8; N]) -> [BoundedVec<Field, N>; 1] {
    // regex: email was meant for @[a-z]+
    let mut s = 0;

    let mut substr0 = BoundedVec::new();

    for i in 0..input.len() {
        let temp = input[i] as Field;
        s = table[s * 256 + input[i] as Field];
        if (s == 22) {
            substr0.push(temp);
        }
    }
    assert((s == 22), f"no match: {s}");
    [substr0]
}

comptime fn make_lookup_table() -> [Field; 5888] {
    let mut table = [0; 5888];
    table[0 * 256 + 101] = 1;
    table[1 * 256 + 109] = 2;
    table[2 * 256 + 97] = 3;
    table[3 * 256 + 105] = 4;
    table[4 * 256 + 108] = 5;
    table[5 * 256 + 32] = 6;
    table[6 * 256 + 119] = 7;
    table[7 * 256 + 97] = 8;
    table[8 * 256 + 115] = 9;
    table[9 * 256 + 32] = 10;
    table[10 * 256 + 109] = 11;
    table[11 * 256 + 101] = 12;
    table[12 * 256 + 97] = 13;
    table[13 * 256 + 110] = 14;
    table[14 * 256 + 116] = 15;
    table[15 * 256 + 32] = 16;
    table[16 * 256 + 102] = 17;
    table[17 * 256 + 111] = 18;
    table[18 * 256 + 114] = 19;
    table[19 * 256 + 32] = 20;
    table[20 * 256 + 64] = 21;
    table[21 * 256 + 97] = 22;
    table[21 * 256 + 98] = 22;
    table[21 * 256 + 99] = 22;
    table[21 * 256 + 100] = 22;
    table[21 * 256 + 101] = 22;
    table[21 * 256 + 102] = 22;
    table[21 * 256 + 103] = 22;
    table[21 * 256 + 104] = 22;
    table[21 * 256 + 105] = 22;
    table[21 * 256 + 106] = 22;
    table[21 * 256 + 107] = 22;
    table[21 * 256 + 108] = 22;
    table[21 * 256 + 109] = 22;
    table[21 * 256 + 110] = 22;
    table[21 * 256 + 111] = 22;
    table[21 * 256 + 112] = 22;
    table[21 * 256 + 113] = 22;
    table[21 * 256 + 114] = 22;
    table[21 * 256 + 115] = 22;
    table[21 * 256 + 116] = 22;
    table[21 * 256 + 117] = 22;
    table[21 * 256 + 118] = 22;
    table[21 * 256 + 119] = 22;
    table[21 * 256 + 120] = 22;
    table[21 * 256 + 121] = 22;
    table[21 * 256 + 122] = 22;
    table[22 * 256 + 97] = 22;
    table[22 * 256 + 98] = 22;
    table[22 * 256 + 99] = 22;
    table[22 * 256 + 100] = 22;
    table[22 * 256 + 101] = 22;
    table[22 * 256 + 102] = 22;
    table[22 * 256 + 103] = 22;
    table[22 * 256 + 104] = 22;
    table[22 * 256 + 105] = 22;
    table[22 * 256 + 106] = 22;
    table[22 * 256 + 107] = 22;
    table[22 * 256 + 108] = 22;
    table[22 * 256 + 109] = 22;
    table[22 * 256 + 110] = 22;
    table[22 * 256 + 111] = 22;
    table[22 * 256 + 112] = 22;
    table[22 * 256 + 113] = 22;
    table[22 * 256 + 114] = 22;
    table[22 * 256 + 115] = 22;
    table[22 * 256 + 116] = 22;
    table[22 * 256 + 117] = 22;
    table[22 * 256 + 118] = 22;
    table[22 * 256 + 119] = 22;
    table[22 * 256 + 120] = 22;
    table[22 * 256 + 121] = 22;
    table[22 * 256 + 122] = 22;
    table[22 * 256 + 0] = 22;
    table[22 * 256 + 1] = 22;
    table[22 * 256 + 2] = 22;
    table[22 * 256 + 3] = 22;
    table[22 * 256 + 4] = 22;
    table[22 * 256 + 5] = 22;
    table[22 * 256 + 6] = 22;
    table[22 * 256 + 7] = 22;
    table[22 * 256 + 8] = 22;
    table[22 * 256 + 9] = 22;
    table[22 * 256 + 10] = 22;
    table[22 * 256 + 11] = 22;
    table[22 * 256 + 12] = 22;
    table[22 * 256 + 13] = 22;
    table[22 * 256 + 14] = 22;
    table[22 * 256 + 15] = 22;
    table[22 * 256 + 16] = 22;
    table[22 * 256 + 17] = 22;
    table[22 * 256 + 18] = 22;
    table[22 * 256 + 19] = 22;
    table[22 * 256 + 20] = 22;
    table[22 * 256 + 21] = 22;
    table[22 * 256 + 22] = 22;
    table[22 * 256 + 23] = 22;
    table[22 * 256 + 24] = 22;
    table[22 * 256 + 25] = 22;
    table[22 * 256 + 26] = 22;
    table[22 * 256 + 27] = 22;
    table[22 * 256 + 28] = 22;
    table[22 * 256 + 29] = 22;
    table[22 * 256 + 30] = 22;
    table[22 * 256 + 31] = 22;
    table[22 * 256 + 32] = 22;
    table[22 * 256 + 33] = 22;
    table[22 * 256 + 34] = 22;
    table[22 * 256 + 35] = 22;
    table[22 * 256 + 36] = 22;
    table[22 * 256 + 37] = 22;
    table[22 * 256 + 38] = 22;
    table[22 * 256 + 39] = 22;
    table[22 * 256 + 40] = 22;
    table[22 * 256 + 41] = 22;
    table[22 * 256 + 42] = 22;
    table[22 * 256 + 43] = 22;
    table[22 * 256 + 44] = 22;
    table[22 * 256 + 45] = 22;
    table[22 * 256 + 46] = 22;
    table[22 * 256 + 47] = 22;
    table[22 * 256 + 48] = 22;
    table[22 * 256 + 49] = 22;
    table[22 * 256 + 50] = 22;
    table[22 * 256 + 51] = 22;
    table[22 * 256 + 52] = 22;
    table[22 * 256 + 53] = 22;
    table[22 * 256 + 54] = 22;
    table[22 * 256 + 55] = 22;
    table[22 * 256 + 56] = 22;
    table[22 * 256 + 57] = 22;
    table[22 * 256 + 58] = 22;
    table[22 * 256 + 59] = 22;
    table[22 * 256 + 60] = 22;
    table[22 * 256 + 61] = 22;
    table[22 * 256 + 62] = 22;
    table[22 * 256 + 63] = 22;
    table[22 * 256 + 64] = 22;
    table[22 * 256 + 65] = 22;
    table[22 * 256 + 66] = 22;
    table[22 * 256 + 67] = 22;
    table[22 * 256 + 68] = 22;
    table[22 * 256 + 69] = 22;
    table[22 * 256 + 70] = 22;
    table[22 * 256 + 71] = 22;
    table[22 * 256 + 72] = 22;
    table[22 * 256 + 73] = 22;
    table[22 * 256 + 74] = 22;
    table[22 * 256 + 75] = 22;
    table[22 * 256 + 76] = 22;
    table[22 * 256 + 77] = 22;
    table[22 * 256 + 78] = 22;
    table[22 * 256 + 79] = 22;
    table[22 * 256 + 80] = 22;
    table[22 * 256 + 81] = 22;
    table[22 * 256 + 82] = 22;
    table[22 * 256 + 83] = 22;
    table[22 * 256 + 84] = 22;
    table[22 * 256 + 85] = 22;
    table[22 * 256 + 86] = 22;
    table[22 * 256 + 87] = 22;
    table[22 * 256 + 88] = 22;
    table[22 * 256 + 89] = 22;
    table[22 * 256 + 90] = 22;
    table[22 * 256 + 91] = 22;
    table[22 * 256 + 92] = 22;
    table[22 * 256 + 93] = 22;
    table[22 * 256 + 94] = 22;
    table[22 * 256 + 95] = 22;
    table[22 * 256 + 96] = 22;
    table[22 * 256 + 123] = 22;
    table[22 * 256 + 124] = 22;
    table[22 * 256 + 125] = 22;
    table[22 * 256 + 126] = 22;
    table[22 * 256 + 127] = 22;
    table[22 * 256 + 128] = 22;
    table[22 * 256 + 129] = 22;
    table[22 * 256 + 130] = 22;
    table[22 * 256 + 131] = 22;
    table[22 * 256 + 132] = 22;
    table[22 * 256 + 133] = 22;
    table[22 * 256 + 134] = 22;
    table[22 * 256 + 135] = 22;
    table[22 * 256 + 136] = 22;
    table[22 * 256 + 137] = 22;
    table[22 * 256 + 138] = 22;
    table[22 * 256 + 139] = 22;
    table[22 * 256 + 140] = 22;
    table[22 * 256 + 141] = 22;
    table[22 * 256 + 142] = 22;
    table[22 * 256 + 143] = 22;
    table[22 * 256 + 144] = 22;
    table[22 * 256 + 145] = 22;
    table[22 * 256 + 146] = 22;
    table[22 * 256 + 147] = 22;
    table[22 * 256 + 148] = 22;
    table[22 * 256 + 149] = 22;
    table[22 * 256 + 150] = 22;
    table[22 * 256 + 151] = 22;
    table[22 * 256 + 152] = 22;
    table[22 * 256 + 153] = 22;
    table[22 * 256 + 154] = 22;
    table[22 * 256 + 155] = 22;
    table[22 * 256 + 156] = 22;
    table[22 * 256 + 157] = 22;
    table[22 * 256 + 158] = 22;
    table[22 * 256 + 159] = 22;
    table[22 * 256 + 160] = 22;
    table[22 * 256 + 161] = 22;
    table[22 * 256 + 162] = 22;
    table[22 * 256 + 163] = 22;
    table[22 * 256 + 164] = 22;
    table[22 * 256 + 165] = 22;
    table[22 * 256 + 166] = 22;
    table[22 * 256 + 167] = 22;
    table[22 * 256 + 168] = 22;
    table[22 * 256 + 169] = 22;
    table[22 * 256 + 170] = 22;
    table[22 * 256 + 171] = 22;
    table[22 * 256 + 172] = 22;
    table[22 * 256 + 173] = 22;
    table[22 * 256 + 174] = 22;
    table[22 * 256 + 175] = 22;
    table[22 * 256 + 176] = 22;
    table[22 * 256 + 177] = 22;
    table[22 * 256 + 178] = 22;
    table[22 * 256 + 179] = 22;
    table[22 * 256 + 180] = 22;
    table[22 * 256 + 181] = 22;
    table[22 * 256 + 182] = 22;
    table[22 * 256 + 183] = 22;
    table[22 * 256 + 184] = 22;
    table[22 * 256 + 185] = 22;
    table[22 * 256 + 186] = 22;
    table[22 * 256 + 187] = 22;
    table[22 * 256 + 188] = 22;
    table[22 * 256 + 189] = 22;
    table[22 * 256 + 190] = 22;
    table[22 * 256 + 191] = 22;
    table[22 * 256 + 192] = 22;
    table[22 * 256 + 193] = 22;
    table[22 * 256 + 194] = 22;
    table[22 * 256 + 195] = 22;
    table[22 * 256 + 196] = 22;
    table[22 * 256 + 197] = 22;
    table[22 * 256 + 198] = 22;
    table[22 * 256 + 199] = 22;
    table[22 * 256 + 200] = 22;
    table[22 * 256 + 201] = 22;
    table[22 * 256 + 202] = 22;
    table[22 * 256 + 203] = 22;
    table[22 * 256 + 204] = 22;
    table[22 * 256 + 205] = 22;
    table[22 * 256 + 206] = 22;
    table[22 * 256 + 207] = 22;
    table[22 * 256 + 208] = 22;
    table[22 * 256 + 209] = 22;
    table[22 * 256 + 210] = 22;
    table[22 * 256 + 211] = 22;
    table[22 * 256 + 212] = 22;
    table[22 * 256 + 213] = 22;
    table[22 * 256 + 214] = 22;
    table[22 * 256 + 215] = 22;
    table[22 * 256 + 216] = 22;
    table[22 * 256 + 217] = 22;
    table[22 * 256 + 218] = 22;
    table[22 * 256 + 219] = 22;
    table[22 * 256 + 220] = 22;
    table[22 * 256 + 221] = 22;
    table[22 * 256 + 222] = 22;
    table[22 * 256 + 223] = 22;
    table[22 * 256 + 224] = 22;
    table[22 * 256 + 225] = 22;
    table[22 * 256 + 226] = 22;
    table[22 * 256 + 227] = 22;
    table[22 * 256 + 228] = 22;
    table[22 * 256 + 229] = 22;
    table[22 * 256 + 230] = 22;
    table[22 * 256 + 231] = 22;
    table[22 * 256 + 232] = 22;
    table[22 * 256 + 233] = 22;
    table[22 * 256 + 234] = 22;
    table[22 * 256 + 235] = 22;
    table[22 * 256 + 236] = 22;
    table[22 * 256 + 237] = 22;
    table[22 * 256 + 238] = 22;
    table[22 * 256 + 239] = 22;
    table[22 * 256 + 240] = 22;
    table[22 * 256 + 241] = 22;
    table[22 * 256 + 242] = 22;
    table[22 * 256 + 243] = 22;
    table[22 * 256 + 244] = 22;
    table[22 * 256 + 245] = 22;
    table[22 * 256 + 246] = 22;
    table[22 * 256 + 247] = 22;
    table[22 * 256 + 248] = 22;
    table[22 * 256 + 249] = 22;
    table[22 * 256 + 250] = 22;
    table[22 * 256 + 251] = 22;
    table[22 * 256 + 252] = 22;
    table[22 * 256 + 253] = 22;
    table[22 * 256 + 254] = 22;
    table[22 * 256 + 255] = 22;

    table
}

fn main(input: [u8; 25], username: [u8; 4]) {
    let substrings = regex_match(input);
    let extracted_substr = substrings[0];

    // Assert the obtained substring equals the expected username
    for i in 0..4 {
        assert(extracted_substr.get(i) == (username[i] as Field));
    }
}

#[test]
fn test_valid() {
    // "email was meant for @noir"
    let input: [u8; 25] = [
        101, 109, 97, 105, 108, 32, 119, 97, 115, 32, 109, 101, 97, 110, 116, 32, 102, 111, 114, 32, 64, 110, 111, 105, 114
    ];

    // Obtain the expected substring
    let substrings = regex_match(input);
    // The implementation is general for extraction of x substrings,
    // but we only have 1 here
    let username = substrings[0];

    // Assert it equals "noir"
    assert(username.get(0) == 110);
    assert(username.get(1) == 111);
    assert(username.get(2) == 105);
    assert(username.get(3) == 114);
}

#[test(should_fail)]
fn test_invalid() {
    // "email was meant for @noir"
    let input: [u8; 22] = [
        119, 114, 111, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 102, 111, 114, 32, 64, 110, 111, 105, 114
    ];

    let username = regex_match(input);
}
