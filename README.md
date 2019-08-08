Ariphmetic parser in Rust
=========================

```rust
let expression = parse("100500*(2+35)-2*5").unwrap();
```
FFI: [testffi.php](testffi.php)

```bash
# build
docker-compose build
docker-compose run php74 cargo build && cbindgen . -o target/testffi.h

#run php
docker-compose run php74 php testffi.php 
```