# ZK-regex + Noir example

In this document, we will present an example of how to use the [zk-regex](https://github.com/zkemail/zk-regex) project to generate [Noir](https://noir-lang.org/) code that proves that an input string satisfies given public regex public regex.

## Requirements:

To follow this example, you should have the following tools installed in your system:
- zk-regex. To install this, follow the instructions presented in the [documentation](https://github.com/zkemail/zk-regex?tab=readme-ov-file#install).
- Noir. You can install this using the [installation instructions](https://noir-lang.org/docs/getting_started/installation/).

## Generating the Noir circuit automatically
Let us suppose that you want to prove that an input string fulfills the regex `m(a|b)+-(c|d)+e` in your own project. First, you need to create your own project using the command
```
$ nargo new own_project
```
This will create a new Noir project inside the `own_project/` folder in which you will put all of your application functionality including the circuit in charge of checking if the input string fulfills the regex pattern.

Now, let us use the zk-regex project to generate the Noir code associated to the regex above. The zk-regex tool allows you to take the provided regex and encoding it into a lookup table that check whether an input fulfills the regex or not. The tool will generate a piece of Noir code that you should include in your own project to create the proof.

The first step is to generate the Noir code for the provided regex. To do this, we execute the following command
```bash
$ zk-regex raw --raw-regex "m(a|b)+-(c|d)+e" --noir-file-path "auto_code.nr"
```

Once the command is executed, it will generate the file `auto_code.nr` that contains the following source code:
```rust
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
```
This source code must be copied into your own code to be used. Let us say that you find appropriate to include the the generated source code into the `main.nr` file in your project. Then you need to copy the content from `auto_code.nr` into `own_project/src/main.nr`. Also you need to modify your `main.nr` file to receive the input string and evaluate the regex:

```rust
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
``` 
Notice that we have called the function `regex_match()` passing the input provided to the `main()` function.

At this point, we have finished to incorporate the code and you can use it in further functionalities. Let us add a test to check that the regex identifies the input string correctly. First, let us add a test where the input corresponds to the regex pattern:
```rust
#[test]
fn test_match() {
    // UTF-8 version of mababaaba-cdddce
    let input = [109, 097, 098, 097, 098, 097, 097, 098, 097, 045, 099, 100, 100, 100, 099, 101];
    regex_match(input);
}
```
When we run the test with the command `nargo test` this test should pass. Also, let us consider a test in which the input string does not match the regex pattern:
```rust
#[test(should_fail)]
fn test_not_match() {
    // UTF-8 version of mabaababaab-cdfe
    let input = [109, 097, 098, 097, 097, 098, 097, 098, 097, 097, 098, 045, 099, 100, 102, 101];
    regex_match(input);
}
```
When we run the command `nargo test`, this last test will also pass.