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
            "regex": {
                "raw": "m(a|b)+-(c|d)+e$"
            },
            "input_size": 16,
            "samples_pass": [
                "mabab-cdcde",
                "ma-ce"
            ],
            "samples_fail": [
                "sdjfalsdjflasjf",
                "slafjsajdflasjd"
            ]
        },
        {
            "regex": {
                "decomposed": [
                    {
                        "is_public": false,
                        "regex_def": "ab"
                    },
                    {
                        "is_public": true,
                        "regex_def": "cd"
                    }
                ]
            },
            "input_size": 16,
            "samples_pass": [
                "abcd"
            ],
            "samples_fail": [
                "abw",
                "cdf"
            ]
        }
    ]
}
```
### Execute tests

```
$ RUST_LOG=info cargo run
```

