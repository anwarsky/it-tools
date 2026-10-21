# it-tools

A small command-line toolbox for everyday developer tasks: generating tokens and IDs, hashing,
bcrypt, Base64 and passphrase-based encryption.

## Build

```sh
cargo build --release
```

The binary is written to `target/release/it-tools`. You can also run it straight from the
source tree with `cargo run -- <command> [options]`.

## Usage

```sh
it-tools <token|id|hash|bcrypt|base64|encrypt|decrypt> [options]
```

If you call a command with the wrong arguments, it prints its own usage.

### token: random string

```sh
it-tools token [length] [-c] [-l] [-d] [-s]
```

| Option   | Meaning                                  |
|----------|------------------------------------------|
| `length` | number of characters (default 12)        |
| `-c`     | capital letters `A-Z`                    |
| `-l`     | lower-case letters `a-z`                 |
| `-d`     | digits `0-9`                             |
| `-s`     | symbols `+-*/()[]{}$@#%&`                |

Without any character-set flags, the token uses capital letters, lower-case letters and digits.

```sh
it-tools token 32 -l -d
```

### id: TSID or UUID

```sh
it-tools id [count] [-s | u1|u3|u4|u5|u6|u7|u8 [name]]
```

- With no options, it prints a TSID as a number. Add `-s` to print the TSID as a string.
- `uN` prints a UUID of version N. `u3` and `u5` need a `name`, which is hashed in the DNS namespace.
- `count` sets how many IDs to print, one per line (default 1).

```sh
it-tools id 5 u7
it-tools id u5 example.com
```

### hash: message digest

```sh
it-tools hash <text> <MD5|SHA|SHA1|SHA224|SHA256|SHA384|SHA512|RIPEMD160>
```

The algorithm name is case-insensitive. The output is a hex string.

```sh
it-tools hash "hello" sha256
```

### bcrypt: hash or verify a password

```sh
it-tools bcrypt <text> [cost]   # hash; cost 4-31, default 10, salt is random
it-tools bcrypt <text> <hash>   # verify; prints true or false
```

When you verify, quote the hash so your shell doesn't expand the `$` signs:

```sh
it-tools bcrypt secret '$2b$10$...'
```

### base64: encode or decode text

```sh
it-tools base64 <encode|decode> <text> [-u]
```

`-u` switches to the URL-safe alphabet (`-` and `_`). Decoding works with or without `=` padding.

### encrypt / decrypt: passphrase encryption

```sh
it-tools encrypt <text> <key> [AES|CHACHA20]
it-tools decrypt <encrypted-text> <key> [AES|CHACHA20]
```

- `AES` uses AES-256-GCM and is the default. `CHACHA20` uses ChaCha20-Poly1305.
- The cipher key is derived from the passphrase with PBKDF2-HMAC-SHA256 (600,000 rounds) and a random salt.
- The output is `base64(salt | nonce | ciphertext)`. To decrypt, use the same key and algorithm.

```sh
it-tools encrypt "top secret" mypass CHACHA20
it-tools decrypt <output-from-above> mypass CHACHA20
```
