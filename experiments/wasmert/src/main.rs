use wasmer_runtime::{error, func, imports, instantiate, Array, Ctx, Func, WasmPtr};

// Implementation for `fn print(ptr: *const u8, len: usize);`
fn wasm_print(c: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
    let mem = c.memory(0);
    let text = ptr.get_utf8_string(mem, len).unwrap();
    print!("{}\n", text);
}

fn main() -> error::Result<()> {
    let wasm_bytes = include_bytes!("../minlib.wasm");
    let import_object = imports! {
        "env" => {
            "print" => func!(wasm_print),
        }
    };
    let instance = instantiate(wasm_bytes, &import_object)?;
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let setup: Func<(), u32> = instance.exports.get("setup")?;
    let draw: Func<u32, WasmPtr<u8, Array>> = instance.exports.get("draw")?;

    let state = setup.call()?;
    print!("\x1B[2J");
    loop {
        print!("\x1B[1;1H");
        let buf = draw.call(state)?;
        let text = buf.get_utf8_string(wasm_instance_memory, 40 * 25).unwrap();
        for r in 0..25 {
            let start = r * 40;
            let stop = start + 40;
            print!("{}\n", &text[start..stop]);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
