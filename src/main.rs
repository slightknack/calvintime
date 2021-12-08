use wasmtime as wt;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasi_common::pipe::ReadPipe;

pub mod plugin;
pub mod handler;

pub use plugin::Plugin;

fn main() {
    let engine = wt::Engine::default();
    let mut linker = wt::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let mut wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();

    wasi.push_preopened_dir(
        Box::new(plugin::Counter::api()),
        plugin::Counter::name(),
    ).unwrap();

    let mut store = wt::Store::new(&engine, wasi);

    let module = wt::Module::from_file(&engine, "demo/target/wasm32-wasi/debug/demo.wasm").unwrap();
    linker.module(&mut store, "", &module).unwrap();
    linker
        .get_default(&mut store, "").unwrap()
        .typed::<(), (), _>(&store).unwrap()
        .call(&mut store, ()).unwrap();
}
