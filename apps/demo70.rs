

use wasmer::{Store, Module, Instance, Value, imports};

fn main() -> anyhow::Result<()> {
    let wasm_bytes = std::fs::read("/Users/andy/CLionProjects/add/target/wasm32-unknown-unknown/release/add.wasm")?;

    let store = Store::default();
    let module = Module::new(&store, &wasm_bytes)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let sum = instance.exports.get_function("sum")?;
    let result = sum.call(&[Value::I32(42),Value::I32(2)])?;
    assert_eq!(result[0], Value::I32(44));
    //dbg!(&result[0].i32());

    Ok(())
}