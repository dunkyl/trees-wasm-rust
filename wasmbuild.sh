wcc -Wl,--no-entry -Wl,--export,tree_sitter_rust -nostartfiles -o ../../rust.wasm -I src src/scanner.c src/parser.c
