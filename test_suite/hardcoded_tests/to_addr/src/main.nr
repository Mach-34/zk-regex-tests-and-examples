// Test: https://github.com/zkemail/zk-regex/blob/main/packages/circom/tests/to_addr.test.js
// This combines 3 generated templates

mod to_all;
mod email_addr;
mod email_addr_with_name;

fn regex_match<let N: u32>(input: [u8; N]) -> BoundedVec<Field, N> {
    let to_all_substrings = to_all::regex_match(input);
    assert(to_all_substrings.len() == 1);
    let to_all_substring = to_all_substrings.get(0);
    let mut to_all_arr: [u8; N] = [0; N];
    for i in 0..N {
        if (i < to_all_substring.len()) {
            to_all_arr[i] = to_all_substring.get(i) as u8;
        }
    }

    // At least 1 of the 2 should return true
    let (email_addr__with_name_substrings, matched_email_with_name_addr) = email_addr_with_name::regex_match(to_all_arr);
    let (email_addr_subtrings, matched_email_addr) = email_addr::regex_match(to_all_arr);

    // If email between "<>" was obtained, return that
    // otherwise return any email that was encountered
    // otherwise this should fail since no valid email was found
    let res = if matched_email_with_name_addr {
        assert(email_addr__with_name_substrings.len() == 1);
        email_addr__with_name_substrings.get(0)
    } else if matched_email_addr {
        assert(email_addr_subtrings.len() == 1);
        email_addr_subtrings.get(0)
    } else {
        assert(false);
        BoundedVec::new()
    };

    res
}

fn main(input: [u8; 100]) {
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

// "adityabisht@gmail.com"
global expected_arr: [u8; 21] = [
    97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109
];

#[test]
fn test_valid_1() {
    let mut expected: BoundedVec<u8, 26> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "to:adityabisht@gmail.com\r\n"
    let input1: [u8; 26] = [
        116, 111, 58, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 13, 10
    ];
    let res1 = regex_match(input1);
    assert_equals_expected(res1, expected);
}

#[test]
fn test_valid_2() {
    let mut expected: BoundedVec<u8, 41> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "to:Aditya Bisht <adityabisht@gmail.com>\r\n"
    let input2: [u8; 41] = [
        116, 111, 58, 65, 100, 105, 116, 121, 97, 32, 66, 105, 115, 104, 116, 32, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res2 = regex_match(input2);
    assert_equals_expected(res2, expected);
}

#[test]
fn test_valid_3() {
    let mut expected: BoundedVec<u8, 45> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "to:dummy@example.com<adityabisht@gmail.com>\r\n"
    let input3: [u8; 45] = [
        116, 111, 58, 100, 117, 109, 109, 121, 64, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res3 = regex_match(input3);

    assert_equals_expected(res3, expected);
}

#[test]
fn test_valid_4() {
    let mut expected: BoundedVec<u8, 44> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "to: \"末神奏宙\" <adityabisht@gmail.com>\r\n"
    let input4: [u8; 44] = [
        116, 111, 58, 32, 34, 229, 191, 160, 231, 137, 135, 232, 191, 148, 229, 185, 180, 34, 32, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];
    let res4 = regex_match(input4);
    assert_equals_expected(res4, expected);
}

#[test]
fn test_valid_5() {
    let mut expected: BoundedVec<u8, 33> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nto:adityabisht@gmail.com\r\n"
    let input5: [u8; 33] = [
        100, 117, 109, 109, 121, 13, 10, 116, 111, 58, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 13, 10
    ];

    let res5 = regex_match(input5);
    assert_equals_expected(res5, expected);
}

#[test]
fn test_valid_6() {
    let mut expected: BoundedVec<u8, 48> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nto:Sora Suegami <adityabisht@gmail.com>\r\n"
    let input6: [u8; 48] = [
        100, 117, 109, 109, 121, 13, 10, 116, 111, 58, 83, 111, 114, 97, 32, 83, 117, 101, 103, 97, 109, 105, 32, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res6 = regex_match(input6);
    assert_equals_expected(res6, expected);
}

#[test]
fn test_valid_7() {
    let mut expected: BoundedVec<u8, 52> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nto:dummy@example.com<adityabisht@gmail.com>\r\n"
    let input7: [u8; 52] = [
        100, 117, 109, 109, 121, 13, 10, 116, 111, 58, 100, 117, 109, 109, 121, 64, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res7 = regex_match(input7);
    assert_equals_expected(res7, expected);
}

#[test]
fn test_valid_8() {
    let mut expected: BoundedVec<u8, 51> = BoundedVec::new();
    for i in 0..21 {
        expected.push(expected_arr[i]);
    }

    // "dummy\r\nto: \"末神奏宙\" <adityabisht@gmail.com>\r\n"
    let input8: [u8; 51] = [
        100, 117, 109, 109, 121, 13, 10, 116, 111, 58, 32, 34, 229, 191, 160, 231, 137, 135, 232, 191, 148, 229, 185, 180, 34, 32, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 62, 13, 10
    ];

    let res8 = regex_match(input8);
    assert_equals_expected(res8, expected);
}

#[test]
fn test_valid_9() {
    let mut expected: BoundedVec<u8, 51> = BoundedVec::new();
    // "adityabisht@gmail.com@dummy.com"
    let expected_arr = [
        97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111,
        109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109
    ];
    for i in 0..31 {
        expected.push(expected_arr[i]);
    }

    // "to:Aditya Bisht <adityabisht@gmail.com@dummy.com>\r\n"
    let input9: [u8; 51] = [
        116, 111, 58, 65, 100, 105, 116, 121, 97, 32, 66, 105, 115, 104, 116, 32, 60, 97, 100, 105, 116, 121, 97, 98, 105, 115, 104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109, 62, 13, 10
    ];

    let res9 = regex_match(input9);
    assert_equals_expected(res9, expected);
}

#[test]
fn test_valid_10() {
    let mut expected: BoundedVec<u8, 40> = BoundedVec::new();
    // @gmail.com@dummy.com
    let expected_arr = [64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109];
    for i in 0..20 {
        expected.push(expected_arr[i]);
    }

    // "to:Aditya Bisht <@gmail.com@dummy.com>\r\n"
    let input10: [u8; 40] = [
        116, 111, 58, 65, 100, 105, 116, 121, 97, 32, 66, 105, 115, 104, 116, 32, 60, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 64, 100, 117, 109, 109, 121, 46, 99, 111, 109, 62, 13, 10
    ];

    let res10 = regex_match(input10);
    assert_equals_expected(res10, expected);
}

#[test(should_fail)]
fn test_invalid_1() {
    let input = [
        115, 117, 98, 106, 101, 99, 116, 58, 116, 111, 58, 97, 100, 105, 116, 121, 97, 98, 105, 115,
        104, 116, 64, 103, 109, 97, 105, 108, 46, 99, 111, 109, 13, 10
    ];
    regex_match(input);
}
