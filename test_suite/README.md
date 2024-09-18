# Test suite for zk-regex + Noir

## Requirements

- Install zk-regex command following the instructions in the documentation.
- Install Noir.

## How to run

### Database

```json
{
    "database": [
        {
            "type": "raw",
            "regex": "m(a|b)+-(c|d)+e$",
            "input_size": 16,
            "samples_pass": [
                "ababababab",
                "aaaavvvaaabba"
            ],
            "samples_fail": [
                "sdjfalsdjflasjf",
                "slafjñsajdflasjd"
            ]
        }
    ]
}
```
### Execute tests

```
$ RUST_LOG=debug cargo run
```

