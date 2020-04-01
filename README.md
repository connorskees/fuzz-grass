# fuzz-grass
Fuzz the grass compiler using AFL

To get started,

```bash
git clone https://connorskees/fuzz-grass
cd fuzz-grass
cargo install --force afl
cargo afl build
cargo afl fuzz -i in -o out target/debug/fuzz-grass
```
