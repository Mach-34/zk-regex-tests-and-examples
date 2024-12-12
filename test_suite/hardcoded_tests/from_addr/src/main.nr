// Test: https://github.com/zkemail/zk-regex/blob/main/packages/circom/tests/from_addr.test.js
// This combines 3 generated templates

mod from_all;
mod email_addr;
mod reversed_bracket;

fn regex_match<let N: u32>(input: [u8; N]) -> BoundedVec<Field, N> {
    let from_all_substrings = from_all::regex_match(input);
    assert(from_all_substrings.len() == 1);
    let from_all_substring = from_all_substrings.get(0);
    let substring_len = from_all_substring.len(); 
    let mut from_all_arr: [u8; N] = [0; N];
    let mut from_all_arr_reversed: [u8; N] = [0; N];
    for i in 0..N {
        if (i < substring_len) {
            from_all_arr[i] = from_all_substring.get(i) as u8;
            from_all_arr_reversed[substring_len-1-i] = from_all_substring.get(i) as u8;
        }
    }

    // At least 1 of the 2 should return true
    // This extracts the first occurence of an email address between brackets <> of the reversed string
    // (this ensure the last occurrence of an email is extracted)
    let (email_addr_reversed_bracket_substrings, matched_email_addr_reversed_bracket) = reversed_bracket::regex_match(from_all_arr_reversed);
    let (email_addr_subtrings, matched_email_addr) = email_addr::regex_match(from_all_arr);
    
    // If email between "<>" was obtained, return that
    // otherwise return any email that was encountered
    // otherwise this should fail since no valid email was found
    let res = if matched_email_addr_reversed_bracket {
        assert(email_addr_reversed_bracket_substrings.len() == 1);
        // The email address was reversed for the check, so we have to reverse it again
        let reversed = email_addr_reversed_bracket_substrings.get(0);
        let mut unreversed: BoundedVec<Field, N> = BoundedVec::new();
        for i in 0..N {
          if (i < reversed.len()) {
            let j = reversed.len() - i -1;
              unreversed.push(reversed.get(j));
          }
        }
        unreversed
    } else if matched_email_addr {
        assert(email_addr_subtrings.len() == 1);
        email_addr_subtrings.get(0)
    } else {
        assert(false);
        BoundedVec::new()
    };

    res
}

fn main(input: [u8; 1024]) {
    let res = regex_match(input);
}

fn assert_equals_expected<let N: u32>(input: BoundedVec<Field, N>, expected: BoundedVec<u8, N>) {
    assert(input.len() == expected.len());
    for i in 0..N {
        if i < input.len() {
            assert(input.get(i) as u8 == expected.get(i));
        }
    }
}

// "suegamisora@gmail.com"
global expected_arr: [u8; 21] = [
    115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109
];

#[test]
fn test_valid_1() { // "from field from beginning case 1"
    let mut expected: BoundedVec<u8, 28> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from:suegamisora@gmail.com\r\n"
    let input: [u8; 28] = [
        102, 114, 111, 109, 58, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 13, 10
    ];
    let res1 = regex_match(input);
    assert_equals_expected(res1, expected);
}

#[test]
fn test_valid_2() { // "from field from beginning case 2"
    let mut expected: BoundedVec<u8, 43> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from:Sora Suegami <suegamisora@gmail.com>\r\n"
    let input2: [u8; 43] = [
        102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];
    let res2 = regex_match(input2);
    assert_equals_expected(res2, expected);
}

#[test]
fn test_valid_3() { // "from field from beginning case 3 (email address as a name)"
    let mut expected: BoundedVec<u8, 47> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from:dummy@example.com<suegamisora@gmail.com>\r\n"
    let input3: [u8; 47] = [
        102, 114, 111, 109, 58, 100, 117, 109, 109, 121, 64, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];
    let res3 = regex_match(input3);

    assert_equals_expected(res3, expected);
}

#[test]
fn test_valid_4() { // "from field from beginning case 4 (non-English string is used as a name)"
    let mut expected: BoundedVec<u8, 46> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from: \"末神奏宙\" <suegamisora@gmail.com>\r\n"
    let input4: [u8; 46] = [
        102, 114, 111, 109, 58, 32, 34, 229, 191, 160, 231, 137, 135, 232, 191, 148, 229, 185, 180, 34, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res4 = regex_match(input4);
    assert_equals_expected(res4, expected);
}

#[test]
fn test_valid_5() { // "from field after new line case 1"
    let mut expected: BoundedVec<u8, 35> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nfrom:suegamisora@gmail.com\r\n"
    let input5: [u8; 35] = [
        100, 117, 109, 109, 121, 13, 10, 102, 114, 111, 109, 58, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 13, 10
    ];

    let res5 = regex_match(input5);
    assert_equals_expected(res5, expected);
}

#[test]
fn test_valid_6() { // "from field after new line case 2"
    let mut expected: BoundedVec<u8, 50> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nfrom:Sora Suegami <suegamisora@gmail.com>\r\n"
    let input6: [u8; 50] = [
        100, 117, 109, 109, 121, 13, 10, 102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res6 = regex_match(input6);
    assert_equals_expected(res6, expected);
}

#[test]
fn test_valid_7() { // "from field after new line case 3 (email address as a name)"
    let mut expected: BoundedVec<u8, 54> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nfrom:dummy@example.com<suegamisora@gmail.com>\r\n"
    let input7: [u8; 54] = [
        100, 117, 109, 109, 121, 13, 10, 102, 114, 111, 109, 58, 100, 117, 109, 109, 121, 64, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res7 = regex_match(input7);
    assert_equals_expected(res7, expected);
}

#[test]
fn test_valid_8() { // "from field after new line case 4 (non-English string is used as a name)"
    let mut expected: BoundedVec<u8, 53> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nfrom: \"末神奏宙\" <suegamisora@gmail.com>\r\n"
    let input8: [u8; 53] = [
        100, 117, 109, 109, 121, 13, 10, 102, 114, 111, 109, 58, 32, 34, 229, 191, 160, 231, 137, 135, 232, 191, 148, 229, 185, 180, 34, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res8 = regex_match(input8);
    assert_equals_expected(res8, expected);
}

#[test]
fn test_valid_9() { // "from field containing @ in the name part"
    let mut expected: BoundedVec<u8, 53> = BoundedVec::new();
    // "suegamisora@gmail.com@dummy.com"
    let expected_arr = [
        115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109
    ];
    for i in 0..31 {
        expected.push(expected_arr[i]);
    }

    // "from:Sora Suegami <suegamisora@gmail.com@dummy.com>\r\n"
    let input9: [u8; 53] = [
        102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109, 62, 13, 10
    ];

    let res9 = regex_match(input9);
    assert_equals_expected(res9, expected);
}

#[test]
fn test_valid_10() { // "from field containing @ in the name part"
    let mut expected: BoundedVec<u8, 42> = BoundedVec::new();
    // @gmail.com@dummy.com
    let expected_arr = [64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109];
    for i in 0..20 {
        expected.push(expected_arr[i]);
    }

    // "from:Sora Suegami <@gmail.com@dummy.com>\r\n"
    let input10: [u8; 42] = [
        102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109, 62, 13, 10
    ];

    let res10 = regex_match(input10);
    assert_equals_expected(res10, expected);
}

#[test]
fn test_valid_11() { // "from field with double <> 1"
    let mut expected: BoundedVec<u8, 61> = BoundedVec::new();
    // "attacker@outlook.com"
    let expected_arr = [97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 117, 116, 108, 111, 111, 107, 46, 99, 111, 109];
    for i in 0..20 {
        expected.push(expected_arr[i]);
    }

    // "from:\"Some name <victim@any-domain>\" <attacker@outlook.com>\r\n"
    let input11: [u8; 61] = [
        102, 114, 111, 109, 58, 34, 83, 111, 109, 101, 32, 110, 97, 109, 101, 32, 60, 118, 105, 99, 116, 105, 109, 64, 
        97, 110, 121, 45, 100, 111, 109, 97, 105, 110, 62, 34, 32, 60, 97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 
        117, 116, 108, 111, 111, 107, 46, 99, 111, 109, 62, 13, 10
    ];

    let res11 = regex_match(input11);
    assert_equals_expected(res11, expected);
}

#[test]
fn test_valid_12() { // "from field with double <> 2"
    let mut expected: BoundedVec<u8, 62> = BoundedVec::new();
    // " attacker@outlook.com"
    let expected_arr = [32, 97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 117, 116, 108, 111, 111, 107, 46, 99, 111, 109];
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from:\"Some name <victim@any-domain>\" < attacker@outlook.com>\r\n"
    let input12: [u8; 62] = [
        102, 114, 111, 109, 58, 34, 83, 111, 109, 101, 32, 110, 97, 109, 101, 32, 60, 118, 105, 99, 116, 105, 109, 64, 
        97, 110, 121, 45, 100, 111, 109, 97, 105, 110, 62, 34, 32, 60, 32, 97, 116, 116, 97, 99, 107, 101, 114, 64, 
        111, 117, 116, 108, 111, 111, 107, 46, 99, 111, 109, 62, 13, 10
    ];

    let res12 = regex_match(input12);
    assert_equals_expected(res12, expected);
}

#[test]
fn test_valid_13() { // "from field with double <> 3"
    let mut expected: BoundedVec<u8, 62> = BoundedVec::new();
    // "attacker@outlook.com "
    let expected_arr = [97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 117, 116, 108, 111, 111, 107, 46, 99, 111, 109, 32];
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "from:\"Some name <victim@any-domain>\" <attacker@outlook.com >\r\n"
    let input13: [u8; 62] = [
        102, 114, 111, 109, 58, 34, 83, 111, 109, 101, 32, 110, 97, 109, 101, 32, 60, 118, 105, 99, 116, 105, 109, 64, 
        97, 110, 121, 45, 100, 111, 109, 97, 105, 110, 62, 34, 32, 60, 97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 
        117, 116, 108, 111, 111, 107, 46, 99, 111, 109, 32, 62, 13, 10
    ];

    let res13 = regex_match(input13);
    assert_equals_expected(res13, expected);
}

#[test]
fn test_valid_14() { // "from field with triple <>"
    let mut expected: BoundedVec<u8, 82> = BoundedVec::new();
    // "attacker@outlook.com"
    let expected_arr = [97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 117, 116, 108, 111, 111, 107, 46, 99, 111, 109];
    for i in 0..20 {
        expected.push(expected_arr[i]);
    }

    // "from:\"Some name <victim1@any-domain<victim1@any-domain>>\" <attacker@outlook.com>\r\n"
    let input14: [u8; 82] = [
        102, 114, 111, 109, 58, 34, 83, 111, 109, 101, 32, 110, 97, 109, 101, 32, 60, 118, 105, 99, 116, 105, 109, 49, 
        64, 97, 110, 121, 45, 100, 111, 109, 97, 105, 110, 60, 118, 105, 99, 116, 105, 109, 49, 64, 97, 110, 121, 45, 
        100, 111, 109, 97, 105, 110, 62, 62, 34, 32, 60, 97, 116, 116, 97, 99, 107, 101, 114, 64, 111, 117, 116, 108, 
        111, 111, 107, 46, 99, 111, 109, 62, 13, 10
    ];

    let res14 = regex_match(input14);
    assert_equals_expected(res14, expected);
}


#[test(should_fail)]
fn test_invalid_1() { // "from field in the invalid field"
// "\r\nto:from:Sora Suegami <suegamisora@gmail.com>\r\n"
    let input = [
        13, 10, 116, 111, 58, 102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];
    regex_match(input);
}

#[test(should_fail)]
fn test_invalid_2() { // "invalid from field with 255"
// "from:Sora Suegami <suegamisora@gmail.com>\r\n"
// with 255 and 49 added before the array
    let input = [
      255, 49,
      102, 114, 111, 109, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 115, 117, 101, 103, 97, 109, 105, 115, 111, 114, 97, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];
    regex_match(input);
}
