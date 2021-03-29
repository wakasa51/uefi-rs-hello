# uefi-rs-hello
## 実行手順
最初に下記を実行
```
rustup install nightly
rustup override add nightly
```

その後に下記を実行
```
cargo +nightly build -Zbuild-std=core --target x86_64-unknown-uefi --release
```

## 参考にしたもの
- Rust で UEFI のハローワールド
  - https://neriring.hatenablog.jp/entry/2020/05/17/133851
- Writing an OS in Rust
  - https://os.phil-opp.com/set-up-rust/#fixing-linker-errors
