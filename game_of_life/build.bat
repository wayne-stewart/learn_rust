

cargo build --target wasm32-unknown-unknown --release

copy target\wasm32-unknown-unknown\release\game_of_life.wasm www\wasm\game_of_life.wasm

