# Example - raw with substring extraction

This example comes from [zk-regex repo](https://github.com/zkemail/zk-regex?tab=readme-ov-file#zk-regex-raw--r-raw_regex--s-substrs_json_path--c-circom_file_path--t-template_name--g-gen_substrs-truefalse). 

Apart from matching a regex, there is the feature to match regex and additionally extract 1 or more substrings. This can be done in the `decomposed` or `raw` setting. In `decomposed` the substrings are defined by public parts of the regex, in `raw` the substrings are defined by their state transitions in an additional json file. 

## Dependencies

- Noir ≥ v0.34.0
- bb >= 0.55.0 (see [version compatibility](https://github.com/AztecProtocol/aztec-packages/blob/master/barretenberg%2Fcpp%2Fsrc%2Fbarretenberg%2Fbb%2Freadme.md#version-compatibility-with-noir) between Noir and bb)
- zk-regex CLI, with Noir support and `gen_substrs` support

Refer to [Noir's docs](https://noir-lang.org/docs/getting_started/installation/) and [BB instructions](https://github.com/AztecProtocol/aztec-packages/blob/master/barretenberg%2Fcpp%2Fsrc%2Fbarretenberg%2Fbb%2Freadme.md#installation) for installation steps.

Currently Noir support for zk-regex is a [WIP](https://github.com/noir-lang/zk-regex). CLI support for extracting substrings (`gen_substrs`), which we need here, is only available on [this branch](https://github.com/hashcloak/noir-zk-regex/tree/features/hc_improvements).  

## Usage

To express the information about what substrings we want to extract, we need to define the state transitions where the substrings will be in the DFA (Deterministic Finite Automaton) that is generated from the regex. To find out this info, follow steps in the [zk-regex repo](https://github.com/olehmisar/zk-regex?tab=readme-ov-file#zk-regex-raw--r-raw_regex--s-substrs_json_path--c-circom_file_path--t-template_name--g-gen_substrs-truefalse). 

The regex we'll work with in this example is `1=(a|b) (2=(b|c)+ )+d` and we'll extract the following substrings:
- at `(a|b)`
- at `(b|c)+` (potentially more than 1)
- at `d`

### Generate Noir code

Create a file `raw_regex_substrs.json` and add information regarding where the substrings should be extracted (follow steps in the [zk-regex repo](https://github.com/olehmisar/zk-regex?tab=readme-ov-file#zk-regex-raw--r-raw_regex--s-substrs_json_path--c-circom_file_path--t-template_name--g-gen_substrs-truefalse) to determine this yourself):
```
{
    "transitions": [
        [
            [
                2,
                3
            ]
        ],
        [
            [
                6,
                7
            ],
            [
                7,
                7
            ]
        ],
        [
            [
                8,
                9
            ]
        ]
    ]
}
```

Run

```bash
$ zk-regex raw -r "1=(a|b) (2=(b|c)+ )+d" -s raw_regex_substrs.json --noir-file-path raw_regex_substrs.nr -g true
```

### Create project

Create a new Noir project.
```bash
$ nargo new raw_example
```

Copy the content from `raw_regex_substrs.nr` into `main.nr`. 

### Test

Here follow 2 test examples that can be added. 

The first test takes as input `1=a 2=bbbbbc d`. It should extract 3 substrings in total. For each of the substrings, it is asserted that the content is correct. 

```rust
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
```

For the second test there will be 5 substrings extracted, since `(2=(b|c)+ )+` occurs three times and for each of them the part in `(b|c)` is extracted.

```rust
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
```

Run tests with
```bash
$ nargo test
```

Additionally you can add a test for an input string that should fail the regex match:
```rust
#[test(should_fail)]
fn test_invalid() {
    // "abc"
    let input: [u8; 3] = [97, 98, 99];

    let res = regex_match(input);
}
```
