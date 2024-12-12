# from_addr test

This test for the circom code uses a circuit that includes 3 templates. The 3 templates have been generated with decomposed regex information (see `/decomposed`):
- "from_all"
- "reversed_bracket"
- "email_addr"

They are combined as follows: "from_all" extracts a substring `s`. The reversed version of `s` is used as the input for "reversed_bracket" and the standard version as input for "email_addr".

- "reversed_bracket" matches an email address between `<>`. Since the input is the reversed string, it take into account the last email that pops up between `<>`
- "email_addr" only matches the email address

If "reversed_bracket" found an email address, that one is reversed and returned. Otherwise the email address found with "email_addr" is returned. If nothing was found, the match fails. 

## Recreate templates

```
zk-regex decomposed -d decomposed/from_all.json --noir-file-path src/from_all.nr -g true

zk-regex decomposed -d decomposed/reversed_bracket.json --noir-file-path src/reversed_bracket.nr -g true

zk-regex decomposed -d decomposed/email_addr.json --noir-file-path src/email_addr.nr -g true
```

Note that `reversed_bracket.nr` and `email_addr.nr` have to be adjusted to return a bool instead of fail the assertion if they don't match the regex. Otherwise we can't execute both functions and see which one returns a substring. 