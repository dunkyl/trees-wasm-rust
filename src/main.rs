use wasmer::{Store, Module, Instance, Value, imports, EngineBuilder, Memory, MemoryType, Pages};
use wasmer::FunctionEnv;
use wasmer_emscripten::*;

fn main() -> anyhow::Result<()> {

    // let compiler_config = Cranelift::default();
    // let engine = EngineBuilder::new(compiler_config);
    // let engine = engine.engine();


    // let mut store = Store::new(&engine);
    let mut store = Store::default();

    let rust_wasm = std::fs::read("./tree-sitter-rust.wasm")?;

    let env = wasmer_emscripten::EmEnv::default();

    
    
    let start = std::time::Instant::now();

    // let module = Module::from_file(&engine, "./rust.wasm")?;
    let module = Module::new(&store, rust_wasm)?;


    

    println!("Module load time: {:?}", start.elapsed());
    
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let mut instance = Instance::new(&mut store, &module, &import_object)?;

    let fnenv = FunctionEnv::new(&mut store, wasmer_emscripten::EmEnv::new());
    let mut emscripten_globals = EmscriptenGlobals::new(&mut store, &fnenv, &module).unwrap();
    
    run_emscripten_instance(&mut instance, fnenv.into_mut(&mut store), &mut emscripten_globals, ".", vec![], None)?;


    let trees_rust = instance.exports.get_function("tree_sitter_rust")?;
    let result = &trees_rust.call(&mut store, &[])?[0];

    println!("Result: {:?}", result);

    let mut buf = [0u8; 65_536*10];

    let smem = instance.exports.get_memory("memory")?;

    smem.view(&store).read(result.i32().unwrap() as u64, &mut buf)?;

    let last_nonzero_byte = buf.iter().rposition(|&x| x != 0).unwrap();

    let (_, u32_view, _)  = unsafe { buf.align_to::<[u32; 8]>() };

    println!("Buf start: {:?}", u32_view[0]);

    println!("Last nonzero @ offset: {:?}", last_nonzero_byte);

    Ok(())
}