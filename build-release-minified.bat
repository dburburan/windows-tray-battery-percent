set RUSTFLAGS=-Zlocation-detail=none -Zfmt-debug=none
cargo +nightly build --release ^
  -Z build-std=std,panic_abort ^
  --target x86_64-pc-windows-msvc
