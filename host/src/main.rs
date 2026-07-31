use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(&engine, "../guest/target/wasm32-unknown-unknown/release/guest.wasm")?;

    let mut linker = Linker::new(&engine);

    let instance = linker.instantiate(&mut store, &module)?;
    let add = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "add")?;

    let result = add.call(&mut store, (2, 3))?;
    println!("Result: {}", result);

    Ok(())
}