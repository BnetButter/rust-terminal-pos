watchmedo auto-restart -d "./src" -p "*.rs" --\
    bash -c "cargo build && ./target/debug/rust-terminal-pos"
