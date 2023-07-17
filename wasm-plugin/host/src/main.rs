use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();

    let Some(file) = std::env::args().nth(1) else {
        anyhow::bail!("USAGE: host <WASM FILE>");
    };

    let module = Module::from_file(&engine, file)?;

    let linker = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .expect("should always be able to inherit args")
        .build();
    let mut store = Store::new(&engine, wasi);

    let instance = linker.instantiate(&mut store, &module)?;

    let Ok(plugin_name_len) = instance.get_typed_func::<(), u32>(&mut store, "plugin_name_len") else {
        anyhow::bail!("Failed to get plugin_name_len");
    };
    let len = plugin_name_len.call(&mut store, ())? as usize;

    let Ok(plugin_name) = instance.get_typed_func::<(), u32>(&mut store, "plugin_name") else {
        anyhow::bail!("Failed to get plugin_name");
    };
    let ptr = plugin_name.call(&mut store, ())? as usize;

    let Some(memory) = instance.get_memory(&mut store, "memory") else {
        anyhow::bail!("Failed to get WASM memory");
    };

    let data = memory.data(&store)[ptr..(ptr + len)].to_vec();
    let name = String::from_utf8(data)?;
    println!("Plugin name: {}", name);

    Ok(())
}
