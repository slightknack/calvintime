use wasmtime::{self as wt};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasi_common::pipe::ReadPipe;

pub mod plugin;
pub mod handler;

fn main() {
    let engine = wt::Engine::default();
    let mut linker = wt::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(ReadPipe::from("hello from stdin!")))
        .inherit_args().unwrap()
        .build();
    let mut store = wt::Store::new(&engine, wasi);

    let module = wt::Module::from_file(&engine, "demo/target/wasm32-wasi/debug/demo.wasm").unwrap();
    linker.module(&mut store, "", &module).unwrap();
    linker
        .get_default(&mut store, "").unwrap()
        .typed::<(), (), _>(&store).unwrap()
        .call(&mut store, ()).unwrap();
}
