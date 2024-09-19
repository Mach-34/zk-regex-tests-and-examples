# Example - decomposed

Using the `decomposed` command, the regex is defined in several parts. Each part is marked private or public. This also allows for the extraction of substrings, since public parts are extracted and returned by the circuit.

In this example the regex is "this email is meant for @[a-z]+" and we're extracting the substring that follows "this email is meant for @". This example exists also for circom in the [zk-regex repo](https://github.com/zkemail/zk-regex?tab=readme-ov-file#zk-regex-decomposed--d-decomposed_regex_path--c-circom_file_path--t-template_name--g-gen_substrs-truefalse). 

## Dependencies

- Noir ≥ v0.34.0
- bb >= 0.55.0 (see [version compatibility](https://github.com/AztecProtocol/aztec-packages/blob/master/barretenberg%2Fcpp%2Fsrc%2Fbarretenberg%2Fbb%2Freadme.md#version-compatibility-with-noir) between Noir and bb)
- zk-regex CLI, with Noir support and `gen_substrs` support

Refer to [Noir's docs](https://noir-lang.org/docs/getting_started/installation/) and [BB instructions](https://github.com/AztecProtocol/aztec-packages/blob/master/barretenberg%2Fcpp%2Fsrc%2Fbarretenberg%2Fbb%2Freadme.md#installation) for installation steps.

Currently Noir support for zk-regex is a [WIP](https://github.com/olehmisar/zk-regex). CLI support for extracting substrings (`gen_substrs`), which we need here, is only available on [this branch](https://github.com/hashcloak/noir-zk-regex/tree/features/gen_substrs). 

## Usage

### Generate Noir code

Create a file called `simple_regex_decomposed.json` and copy this content:

```
{
     "parts":[
         {
             "is_public": false,
             "regex_def": "email was meant for @"
         },
         {
             "is_public": true,
             "regex_def": "[a-z]+"
         }
     ]
}
```

Run the following command:
```
$ zk-regex decomposed -d simple_regex_decomposed.json --noir-file-path simple_regex.nr -g true
```

Command dissection:
- `-d` indicates the json file that contains the regex parts
- `--noir-file-path` where the Noir output code should go
- `-g` whether 1 or more substrings should be extracted

This will generate the Noir code in `simple_regex.nr`. 
This consists of a `regex_match` function which returns a `BoundedVec` which contains the substring. 

See for a reference of the code in the example project here. 

### Create project

Create a new Noir project.
```
$ nargo new decomposed_example
```

Copy the content from `simple_regex.nr` into `main.nr`. 

### Test

Now we can add test cases to check whether the implementation is working correctly. Example of a test that should pass:

```rust
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
```

Example of a test that should fail:
[119, 114, 111, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 102, 111, 114, 32, 64, 110, 111, 105, 114]
```rust
#[test(should_fail)]
fn test_invalid() {
    // "email was meant for @noir"
    let input: [u8; 22] = [119, 114, 111, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 102, 111, 114, 32, 64, 110, 111, 105, 114];

    let username = regex_match(input);
}
```

Run tests with
```bash
$ nargo test
```

### Prove & verify

Let's use the input of the valid test for proving and verifying as well. Create a `main` function:
```rust
fn main(input: [u8; 25], username: [u8; 4]) {
    let substrings = regex_match(input);
    let extracted_substr = substrings[0];

    // Assert the obtained substring equals the expected username
    for i in 0..4 {
        assert(extracted_substr.get(i) == (username[i] as Field));
    }
}
```

Run `nargo check` to check for errors:

```bash
nargo check
```
In `Prover.toml` add the input string and the username. The predefined values are equal to the ones as in `test_valid`.

Then execute it, and prove it i.e. with barretenberg:

```bash
nargo execute dec_example
bb prove -b ./target/decomposed_example.json -w ./target/dec_example.gz -o ./target/proof
```

### Verify it

To verify, we need to export the verification key:

```bash
bb write_vk -b ./target/decomposed_example.json -o ./target/vk
```

And verify:

```bash
bb verify -k ./target/vk -p ./target/proof
```

If there is no output, verification was successful.