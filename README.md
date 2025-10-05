# URLX

Easily decode and read url query parmas

```bash
urlx https://example.com?name=John%20Doe&age=30
```

Will output:
```plaintext
URL: https://example.com
name: John Doe
age: 30
```

## Prerequisites
- Rust and Cargo installed. You can install them from [here](https://www.rust-lang.org/tools/install).

## Install
1. Clone the repository
```bash
git clone https://github.com/eli-front/urlx
```
2. Navigate to the project directory
```bash
cd urlx
```
3. Build
```bash
cargo build --release
```
4. Install
```bash
cargo install --path .
```


