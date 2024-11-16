### Basic project of interpreting Rust code into a web application using WebAssembly

![Rust Version](https://img.shields.io/badge/rust-1.82.0%20-green)
![Build Status](https://github.com/chemyl/tax_app_webassembly/actions/workflows/rust.yml/badge.svg)

- create library crate -> `cargo new project --lib`
- add dependency to `cargo.toml`-> `wasm-bindgen = "0.2.+"`
- add library annotation for compiler ->` [lib] crate-type = ["cdylib"]`
- include wasm prelude inside `lib.rs`
- create public function with necessary logic in `lib.rs`
- create `index.html` file with simple html template with header and `style.css` inclusion in project root
- create `styles.css` style file
- create skeleton inside `index.html` and extend `script block` with logic of working with wasm inclusion
- install wasm-pack -> `cargo install wasm-pack`
- build project with wasm-pack -> `wasm-pack build --target web`
- include web server -> `npm install -g http-server`
- start server in project root folder -> `http-server .`
- switch to `localhost`

![launcher window](https://github.com/chemyl/tax_app_webassembly/blob/master/img.png)
