

rem cargo build --target wasm32-unknown-unknown

rem wasm-bindgen target/wasm32-unknown-unknown/debug/hello_wasm.wasm --out-dir ./www/wasm


cargo build --target wasm32-unknown-unknown --release

rem copy target\wasm32-unknown-unknown\debug\hello_wasm.wasm www\wasm\hello_wasm.wasm
copy target\wasm32-unknown-unknown\release\hello_wasm.wasm www\wasm\hello_wasm.wasm