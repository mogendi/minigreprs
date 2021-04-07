### Running
```bash
cargo run  [search-string] [target-file]
```
```bash 
cargo test [search-string] [target-file]
```

### Installing
```bash
git clone https://github.com/mogendi/minigreprs.git
cd minigreprs 
cargo install --path .
``` 
For this to work, ensure you cargo binaries are added to you PATH variable:
```bash
export PATH="~/.cargo/bin: $PATH"
```

Then you can use it like above:
``` bash
minigrep [search-string] [target-file]
```
