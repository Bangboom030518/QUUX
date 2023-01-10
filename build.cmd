@echo off
tailwindcss -i ./src/style.css -o ./dist/output.css -c ./tailwind.config.js --minify && wasm-pack build --target web --out-dir assets && cargo run