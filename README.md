[[en](./doc/README_en.md)]

# `ip-part` - 用于IP段处理的工具

## 使用

在 `Cargo.toml` 中添加依赖:

```toml
[dependencies]
ip-part = "0.1.0"
```

## 示例

```rust
let ip = IpPart::new("192.168.1.1/24").unwrap();

println!("size {}", ip.size());

println!("{:?}", ip.list());
```
