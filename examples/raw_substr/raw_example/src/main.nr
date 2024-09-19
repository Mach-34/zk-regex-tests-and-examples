global table = comptime { make_lookup_table() };
pub fn regex_match<let N: u32>(input: [u8; N]) -> Vec<BoundedVec<Field, N>> {
    // regex: 1=(a|b) (2=(b|c)+ )+d
    let mut substrings: Vec<BoundedVec<Field, N>> = Vec::new();
    // Workaround for pop bug with Vec
    let mut substr_count = 0;

    // "Previous" state
    let mut s: Field = 0;
    // "Next"/upcoming state
    let mut s_next: Field = 0;

    let mut consecutive_substr = 0;

    for i in 0..input.len() {
        let temp = input[i] as Field;
        s_next = table[s * 256 + temp];
        // Fill up substrings
        if ((s_next == 3) & (consecutive_substr == 0)) {
            let mut substr0 = BoundedVec::new();
            substr0.push(temp);
            substrings.push(substr0);
            consecutive_substr = 1;
            substr_count += 1;
        } else if ((s_next == 3) & (s == 3)) {
            let mut current: BoundedVec<Field, N> = substrings.get(substr_count - 1);
            current.push(temp);
            substrings.set(substr_count - 1, current);
        } else if (s == 3) {
            consecutive_substr = 0;
        }
        if ((s_next == 7) & (consecutive_substr == 0)) {
            let mut substr0 = BoundedVec::new();
            substr0.push(temp);
            substrings.push(substr0);
            consecutive_substr = 1;
            substr_count += 1;
        } else if ((s_next == 7) & (s == 7)) {
            let mut current: BoundedVec<Field, N> = substrings.get(substr_count - 1);
            current.push(temp);
            substrings.set(substr_count - 1, current);
        } else if (s == 7) {
            consecutive_substr = 0;
        }
        if ((s_next == 9) & (consecutive_substr == 0)) {
            let mut substr0 = BoundedVec::new();
            substr0.push(temp);
            substrings.push(substr0);
            consecutive_substr = 1;
            substr_count += 1;
        } else if ((s_next == 9) & (s == 9)) {
            let mut current: BoundedVec<Field, N> = substrings.get(substr_count - 1);
            current.push(temp);
            substrings.set(substr_count - 1, current);
        } else if (s == 9) {
            consecutive_substr = 0;
        }
        s = s_next;
    }
    assert((s == 9), f"no match: {s}");
    substrings
}

comptime fn make_lookup_table() -> [Field; 2560] {
    let mut table = [0; 2560];
    table[0 * 256 + 49] = 1;
    table[1 * 256 + 61] = 2;
    table[2 * 256 + 97] = 3;
    table[2 * 256 + 98] = 3;
    table[3 * 256 + 32] = 4;
    table[4 * 256 + 50] = 5;
    table[5 * 256 + 61] = 6;
    table[6 * 256 + 98] = 7;
    table[6 * 256 + 99] = 7;
    table[7 * 256 + 98] = 7;
    table[7 * 256 + 99] = 7;
    table[7 * 256 + 32] = 8;
    table[8 * 256 + 50] = 5;
    table[8 * 256 + 100] = 9;
    table[9 * 256 + 0] = 9;
    table[9 * 256 + 1] = 9;
    table[9 * 256 + 2] = 9;
    table[9 * 256 + 3] = 9;
    table[9 * 256 + 4] = 9;
    table[9 * 256 + 5] = 9;
    table[9 * 256 + 6] = 9;
    table[9 * 256 + 7] = 9;
    table[9 * 256 + 8] = 9;
    table[9 * 256 + 9] = 9;
    table[9 * 256 + 10] = 9;
    table[9 * 256 + 11] = 9;
    table[9 * 256 + 12] = 9;
    table[9 * 256 + 13] = 9;
    table[9 * 256 + 14] = 9;
    table[9 * 256 + 15] = 9;
    table[9 * 256 + 16] = 9;
    table[9 * 256 + 17] = 9;
    table[9 * 256 + 18] = 9;
    table[9 * 256 + 19] = 9;
    table[9 * 256 + 20] = 9;
    table[9 * 256 + 21] = 9;
    table[9 * 256 + 22] = 9;
    table[9 * 256 + 23] = 9;
    table[9 * 256 + 24] = 9;
    table[9 * 256 + 25] = 9;
    table[9 * 256 + 26] = 9;
    table[9 * 256 + 27] = 9;
    table[9 * 256 + 28] = 9;
    table[9 * 256 + 29] = 9;
    table[9 * 256 + 30] = 9;
    table[9 * 256 + 31] = 9;
    table[9 * 256 + 32] = 9;
    table[9 * 256 + 33] = 9;
    table[9 * 256 + 34] = 9;
    table[9 * 256 + 35] = 9;
    table[9 * 256 + 36] = 9;
    table[9 * 256 + 37] = 9;
    table[9 * 256 + 38] = 9;
    table[9 * 256 + 39] = 9;
    table[9 * 256 + 40] = 9;
    table[9 * 256 + 41] = 9;
    table[9 * 256 + 42] = 9;
    table[9 * 256 + 43] = 9;
    table[9 * 256 + 44] = 9;
    table[9 * 256 + 45] = 9;
    table[9 * 256 + 46] = 9;
    table[9 * 256 + 47] = 9;
    table[9 * 256 + 48] = 9;
    table[9 * 256 + 49] = 9;
    table[9 * 256 + 50] = 9;
    table[9 * 256 + 51] = 9;
    table[9 * 256 + 52] = 9;
    table[9 * 256 + 53] = 9;
    table[9 * 256 + 54] = 9;
    table[9 * 256 + 55] = 9;
    table[9 * 256 + 56] = 9;
    table[9 * 256 + 57] = 9;
    table[9 * 256 + 58] = 9;
    table[9 * 256 + 59] = 9;
    table[9 * 256 + 60] = 9;
    table[9 * 256 + 61] = 9;
    table[9 * 256 + 62] = 9;
    table[9 * 256 + 63] = 9;
    table[9 * 256 + 64] = 9;
    table[9 * 256 + 65] = 9;
    table[9 * 256 + 66] = 9;
    table[9 * 256 + 67] = 9;
    table[9 * 256 + 68] = 9;
    table[9 * 256 + 69] = 9;
    table[9 * 256 + 70] = 9;
    table[9 * 256 + 71] = 9;
    table[9 * 256 + 72] = 9;
    table[9 * 256 + 73] = 9;
    table[9 * 256 + 74] = 9;
    table[9 * 256 + 75] = 9;
    table[9 * 256 + 76] = 9;
    table[9 * 256 + 77] = 9;
    table[9 * 256 + 78] = 9;
    table[9 * 256 + 79] = 9;
    table[9 * 256 + 80] = 9;
    table[9 * 256 + 81] = 9;
    table[9 * 256 + 82] = 9;
    table[9 * 256 + 83] = 9;
    table[9 * 256 + 84] = 9;
    table[9 * 256 + 85] = 9;
    table[9 * 256 + 86] = 9;
    table[9 * 256 + 87] = 9;
    table[9 * 256 + 88] = 9;
    table[9 * 256 + 89] = 9;
    table[9 * 256 + 90] = 9;
    table[9 * 256 + 91] = 9;
    table[9 * 256 + 92] = 9;
    table[9 * 256 + 93] = 9;
    table[9 * 256 + 94] = 9;
    table[9 * 256 + 95] = 9;
    table[9 * 256 + 96] = 9;
    table[9 * 256 + 97] = 9;
    table[9 * 256 + 98] = 9;
    table[9 * 256 + 99] = 9;
    table[9 * 256 + 100] = 9;
    table[9 * 256 + 101] = 9;
    table[9 * 256 + 102] = 9;
    table[9 * 256 + 103] = 9;
    table[9 * 256 + 104] = 9;
    table[9 * 256 + 105] = 9;
    table[9 * 256 + 106] = 9;
    table[9 * 256 + 107] = 9;
    table[9 * 256 + 108] = 9;
    table[9 * 256 + 109] = 9;
    table[9 * 256 + 110] = 9;
    table[9 * 256 + 111] = 9;
    table[9 * 256 + 112] = 9;
    table[9 * 256 + 113] = 9;
    table[9 * 256 + 114] = 9;
    table[9 * 256 + 115] = 9;
    table[9 * 256 + 116] = 9;
    table[9 * 256 + 117] = 9;
    table[9 * 256 + 118] = 9;
    table[9 * 256 + 119] = 9;
    table[9 * 256 + 120] = 9;
    table[9 * 256 + 121] = 9;
    table[9 * 256 + 122] = 9;
    table[9 * 256 + 123] = 9;
    table[9 * 256 + 124] = 9;
    table[9 * 256 + 125] = 9;
    table[9 * 256 + 126] = 9;
    table[9 * 256 + 127] = 9;
    table[9 * 256 + 128] = 9;
    table[9 * 256 + 129] = 9;
    table[9 * 256 + 130] = 9;
    table[9 * 256 + 131] = 9;
    table[9 * 256 + 132] = 9;
    table[9 * 256 + 133] = 9;
    table[9 * 256 + 134] = 9;
    table[9 * 256 + 135] = 9;
    table[9 * 256 + 136] = 9;
    table[9 * 256 + 137] = 9;
    table[9 * 256 + 138] = 9;
    table[9 * 256 + 139] = 9;
    table[9 * 256 + 140] = 9;
    table[9 * 256 + 141] = 9;
    table[9 * 256 + 142] = 9;
    table[9 * 256 + 143] = 9;
    table[9 * 256 + 144] = 9;
    table[9 * 256 + 145] = 9;
    table[9 * 256 + 146] = 9;
    table[9 * 256 + 147] = 9;
    table[9 * 256 + 148] = 9;
    table[9 * 256 + 149] = 9;
    table[9 * 256 + 150] = 9;
    table[9 * 256 + 151] = 9;
    table[9 * 256 + 152] = 9;
    table[9 * 256 + 153] = 9;
    table[9 * 256 + 154] = 9;
    table[9 * 256 + 155] = 9;
    table[9 * 256 + 156] = 9;
    table[9 * 256 + 157] = 9;
    table[9 * 256 + 158] = 9;
    table[9 * 256 + 159] = 9;
    table[9 * 256 + 160] = 9;
    table[9 * 256 + 161] = 9;
    table[9 * 256 + 162] = 9;
    table[9 * 256 + 163] = 9;
    table[9 * 256 + 164] = 9;
    table[9 * 256 + 165] = 9;
    table[9 * 256 + 166] = 9;
    table[9 * 256 + 167] = 9;
    table[9 * 256 + 168] = 9;
    table[9 * 256 + 169] = 9;
    table[9 * 256 + 170] = 9;
    table[9 * 256 + 171] = 9;
    table[9 * 256 + 172] = 9;
    table[9 * 256 + 173] = 9;
    table[9 * 256 + 174] = 9;
    table[9 * 256 + 175] = 9;
    table[9 * 256 + 176] = 9;
    table[9 * 256 + 177] = 9;
    table[9 * 256 + 178] = 9;
    table[9 * 256 + 179] = 9;
    table[9 * 256 + 180] = 9;
    table[9 * 256 + 181] = 9;
    table[9 * 256 + 182] = 9;
    table[9 * 256 + 183] = 9;
    table[9 * 256 + 184] = 9;
    table[9 * 256 + 185] = 9;
    table[9 * 256 + 186] = 9;
    table[9 * 256 + 187] = 9;
    table[9 * 256 + 188] = 9;
    table[9 * 256 + 189] = 9;
    table[9 * 256 + 190] = 9;
    table[9 * 256 + 191] = 9;
    table[9 * 256 + 192] = 9;
    table[9 * 256 + 193] = 9;
    table[9 * 256 + 194] = 9;
    table[9 * 256 + 195] = 9;
    table[9 * 256 + 196] = 9;
    table[9 * 256 + 197] = 9;
    table[9 * 256 + 198] = 9;
    table[9 * 256 + 199] = 9;
    table[9 * 256 + 200] = 9;
    table[9 * 256 + 201] = 9;
    table[9 * 256 + 202] = 9;
    table[9 * 256 + 203] = 9;
    table[9 * 256 + 204] = 9;
    table[9 * 256 + 205] = 9;
    table[9 * 256 + 206] = 9;
    table[9 * 256 + 207] = 9;
    table[9 * 256 + 208] = 9;
    table[9 * 256 + 209] = 9;
    table[9 * 256 + 210] = 9;
    table[9 * 256 + 211] = 9;
    table[9 * 256 + 212] = 9;
    table[9 * 256 + 213] = 9;
    table[9 * 256 + 214] = 9;
    table[9 * 256 + 215] = 9;
    table[9 * 256 + 216] = 9;
    table[9 * 256 + 217] = 9;
    table[9 * 256 + 218] = 9;
    table[9 * 256 + 219] = 9;
    table[9 * 256 + 220] = 9;
    table[9 * 256 + 221] = 9;
    table[9 * 256 + 222] = 9;
    table[9 * 256 + 223] = 9;
    table[9 * 256 + 224] = 9;
    table[9 * 256 + 225] = 9;
    table[9 * 256 + 226] = 9;
    table[9 * 256 + 227] = 9;
    table[9 * 256 + 228] = 9;
    table[9 * 256 + 229] = 9;
    table[9 * 256 + 230] = 9;
    table[9 * 256 + 231] = 9;
    table[9 * 256 + 232] = 9;
    table[9 * 256 + 233] = 9;
    table[9 * 256 + 234] = 9;
    table[9 * 256 + 235] = 9;
    table[9 * 256 + 236] = 9;
    table[9 * 256 + 237] = 9;
    table[9 * 256 + 238] = 9;
    table[9 * 256 + 239] = 9;
    table[9 * 256 + 240] = 9;
    table[9 * 256 + 241] = 9;
    table[9 * 256 + 242] = 9;
    table[9 * 256 + 243] = 9;
    table[9 * 256 + 244] = 9;
    table[9 * 256 + 245] = 9;
    table[9 * 256 + 246] = 9;
    table[9 * 256 + 247] = 9;
    table[9 * 256 + 248] = 9;
    table[9 * 256 + 249] = 9;
    table[9 * 256 + 250] = 9;
    table[9 * 256 + 251] = 9;
    table[9 * 256 + 252] = 9;
    table[9 * 256 + 253] = 9;
    table[9 * 256 + 254] = 9;
    table[9 * 256 + 255] = 9;

    table
}

#[test]
fn test_substr1() {
    // Input for "1=a 2=bbbbbc d"
    let input = [49, 61, 97, 32, 50, 61, 98, 98, 98, 98, 98, 99, 32, 100];
    // This should contain 3 substrings: "a", "bbbbbc", and "d"
    let res = regex_match(input);
    assert(res.len() == 3);

    let substr0 = res.get(0); // "a"
    let substr1 = res.get(1); // "bbbbbc"
    let substr2 = res.get(2); // "d"

    // Check the characters in each substring
    assert(substr0.get(0) == 97); // 'a'

    assert(substr1.get(0) == 98); // 'b'
    assert(substr1.get(5) == 99); // 'c'

    assert(substr2.get(0) == 100); // 'd'
}

#[test]
fn test_substr2() {
    // Input for "1=b 2=bbcb 2=c 2=bb d"
    let input = [49, 61, 98, 32, 50, 61, 98, 98, 99, 98, 32, 50, 61, 99, 32, 50, 61, 98, 98, 32, 100];
    // This should contain 5 substrings: "b", "bbcb", "c", "bb", and "d"
    let res = regex_match(input);
    assert(res.len() == 5);

    let substr0 = res.get(0); // "b"
    let substr1 = res.get(1); // "bbcb"
    let substr2 = res.get(2); // "c"
    let substr3 = res.get(3); // "bb"
    let substr4 = res.get(4); // "d"

    // Check the characters in each substring
    assert(substr0.get(0) == 98); // 'b'

    assert(substr1.get(0) == 98); // 'b'
    assert(substr1.get(3) == 98); // 'b'
    assert(substr1.get(2) == 99); // 'c'

    assert(substr2.get(0) == 99); // 'c'

    assert(substr3.get(0) == 98); // 'b'
    assert(substr3.get(1) == 98); // 'b'

    assert(substr4.get(0) == 100); // 'd'
}

#[test(should_fail)]
fn test_invalid() {
    // "abc"
    let input: [u8; 3] = [97, 98, 99];

    let res = regex_match(input);
}
