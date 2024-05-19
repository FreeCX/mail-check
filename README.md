# mail-check
Simple mail check app

## How to use
1. create [config.toml](./config/config.toml)
2. add account
```bash
$ mail-check -c config.toml add --login "user@example.com" --domain "imap.example.com" --port 993
Enter password for user@example.com:
# password stored in keyring
# login, domain and port in config.toml
```
3. run
```bash
$ mail-check -c config.toml
```
4. (optional) remove account
```bash
$ mail-check -c config.toml remove --login "user@example.com"
```

## Scheduled launch
See [mail-check.service](./config/mail-check.service) and [mail-check.timer](./config/mail-check.timer).

# License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
