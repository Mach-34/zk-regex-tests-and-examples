global table = make_lookup_table();
pub fn regex_match<let N: u32>(input: [u8; N]) {
    let mut s = 0;
    for i in 0..input.len() {
        s = table[s * 256 + input[i] as Field];
    }
    assert_eq(s, 5, f"no match: {s}");
}

comptime fn make_lookup_table() -> [Field; 1536] {
    let mut table = [0; 1536];
    table[0 * 256 + 109] = 1;
    table[1 * 256 + 97] = 2;
    table[1 * 256 + 98] = 2;
    table[2 * 256 + 97] = 2;
    table[2 * 256 + 98] = 2;
    table[2 * 256 + 45] = 3;
    table[3 * 256 + 99] = 4;
    table[3 * 256 + 100] = 4;
    table[4 * 256 + 99] = 4;
    table[4 * 256 + 100] = 4;
    table[4 * 256 + 101] = 5;

    for i in 0..256 {
        table[5 * 256 + i] = 5;
    }
    table
}

fn main(input: [u8; 16]) {
    regex_match(input);
}

#[test]
fn test_match() {
    // UTF-8 version of mababaaba-cdddce
    let input = [109, 097, 098, 097, 098, 097, 097, 098, 097, 045, 099, 100, 100, 100, 099, 101];
    regex_match(input);
}

#[test(should_fail)]
fn test_not_match() {
    // UTF-8 version of mabaababaab-cdfe
    let input = [109, 097, 098, 097, 097, 098, 097, 098, 097, 097, 098, 045, 099, 100, 102, 101];
    regex_match(input);
}
