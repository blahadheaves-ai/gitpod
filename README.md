# gitpodrust-wasm-sandbox/
│
├── guest/        # Rust code that compiles to WASM
│   └── src/
│       └── lib.rs
│
├── host/         # Rust code that runs the WASM sandbox
│   └── src/
│       └── main.rs
│
└── .gitpod.yml   # Cloud dev environment config
host/
├── Cargo.toml
├── build.rs                   <-- Compiles thermite_teardown.c
└── src/
    ├── main.rs                <-- Host engine entry point
    ├── teardown.rs            <-- Rust FFI bindings & ThermiteTeardown wrapper
    └── thermite_teardown.c    <-- Low-level C memory wipe & unmap implementation