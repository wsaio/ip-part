# `ip-part` - ip tool

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ip-part = "0.1.0"
```

## Example

```rust
let ip = IpPart::new("192.168.1.1/24").unwrap();

println!("size {}", ip.size());

println!("{:?}", ip.list());
```
