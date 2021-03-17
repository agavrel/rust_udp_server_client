while inotifywait -e close_write src/main.rs; do  cargo run --bin crypto; done
