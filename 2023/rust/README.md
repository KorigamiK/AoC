# AoC - 2023 (Rust)

## Running Solutions

```sh
Ξ rust ▶ cargo run --bin day01 --release < input.txt
```

## Populating Test Cases

```sh
ln -s /home/origami/Dev/projects/AoC/2023/*.testcases ./src/bin
# convert the names to lowercase
for f in ./src/bin/*.testcases; do mv "$f" "$(echo "$f" | tr '[:upper:]' '[:lower:]')"; done
```
