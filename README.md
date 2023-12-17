# PIN
Command line tool for swedish personal identity numbers (personnummer)

# Build
```bash
git clone https://github.com/Square-face/pin
cd pin
cargo build --release
```
The executable can then be found in `target/release/pin`

# Usage
## Single use
```
>>> pin 201001012382
201001012382 is valid
```

## Using stdin
```
>>> pin
201001012382
201001012382 is valid
```
