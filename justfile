default:
    just --list

wasm-pack:
    wasm-pack build --target web --release
