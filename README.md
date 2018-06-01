## Pichu

A very minimal log shipper. It ships logs via a tcp interface to other server.

## Installation

```
cd pichu/
cargo build --release

```

## Usage

```
USAGE:
    pichu <file> <address>

eg:
   pichu django.log myserver:8080
```

## License
MIT