# from_addr test

This test for the circom code uses a circuit that includes 3 templates. The 3 templates have been generated with decomposed regex information (see `/decomposed`):
- "from_all"
- "email_addr_with_name"
- "email_addr"

They are combined as follows: "from_all" extracts a substring `s`. Then `s` is used as the input for both "email_addr_with_name" and "email_addr".

- "email_addr_with_name" matches an email address between `<>` 
- "email_addr" only matches the email address

If "email_addr_with_name" found an email address, that one is returned. Otherwise the email address found with "email_addr" is returned. If nothing was found, the match fails. 

## Recreate templates

```
zk-regex decomposed -d decomposed/from_all.json --noir-file-path src/from_all.nr -g true

zk-regex decomposed -d decomposed/email_addr_with_name.json --noir-file-path src/email_addr_with_name.nr -g true

zk-regex decomposed -d decomposed/email_addr.json --noir-file-path src/email_addr.nr -g true
```

Note that `email_addr_with_name.nr` and `email_addr.nr` have to be adjusted to return a bool instead of fail the assertion if they don't match the regex. Otherwise we can't execute both functions and see which one returns a substring. 