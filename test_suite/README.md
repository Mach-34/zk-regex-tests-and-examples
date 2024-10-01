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

## Limitations

For some regexes the random sampling is not possible, because the sampling library is limited. For example the end anchor (`$`) is not supported. 

Random sample testing for the `gen_substrs` setting is only support for `decomposed`. In the `raw` setting, the substrings are determined via a json file that contains the transition information. Determining what the substring parts are, would be quite involved since it requires building the DFA. 