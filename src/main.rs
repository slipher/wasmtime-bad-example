
extern crate wasmtime;
extern crate wasmtime_runtime;

use std::collections::HashMap;
use wasmtime::Val;

thread_local!(static inst: std::cell::RefCell<Option<wasmtime::Instance>> = std::cell::RefCell::new(None));

#[allow(unused_variables)]
fn inst_mem() -> wasmtime_runtime::VMMemoryDefinition {
    inst.with(|i| {
    match (*i.borrow()).as_ref().unwrap().get_wasmtime_memory().unwrap() {
        wasmtime_runtime::Export::Memory {definition,vmctx,memory} => unsafe {*definition},
        _ => panic!("memory not found"),
    }
    })
}

fn inst_call(f: &str, args: &[Val]) -> Box<[Val]> {
    inst.with(|i| {
        i.borrow().as_ref().unwrap().find_export_by_name(f).unwrap().func().unwrap().borrow().call(args).unwrap()
    })
}

fn inst_mem_slice(ptr: usize, len: usize) -> &'static mut[u8] {
    let mem = inst_mem();
    if ptr > mem.current_length || len > mem.current_length - ptr {
        panic!("bad vm memory range");
    }
    return unsafe { std::slice::from_raw_parts_mut(mem.base.offset(ptr as isize), len) };
}

struct LogF {}
impl wasmtime::Callable for LogF {
    fn call(&self, args: &[Val], _: &mut[Val]) -> Result<(), wasmtime::Trap> {
        match &args[..] {
            [Val::I32(ptr), Val::I32(len)] => {
                let string = inst_mem_slice(*ptr as usize, *len as usize);
                println!("{}", std::str::from_utf8(&string).unwrap());
            },
            _ => panic!("nthnth"),
        };
        Ok(())
    }
}

fn coerce_fn(et: &wasmtime::ExternType) -> wasmtime::FuncType {
    match et {
        wasmtime::ExternType::Func(ft) => ft.clone(),
        _ => panic!("import isn't a function"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    }
    let binary: Vec<u8> = std::fs::read(&args[1]).unwrap();
    let store = wasmtime::Store::default();
    let module = wasmtime::Module::new(&store, &binary).unwrap();

    let mut provided_fns: HashMap<String, std::rc::Rc<dyn wasmtime::Callable> > = std::collections::HashMap::new();
    let wasmlog = std::rc::Rc::new(LogF {});

    provided_fns.insert(String::from("env/WasmLog"), wasmlog);

    let mut imps: Vec<wasmtime::Extern> = vec!();
    for import_type in module.imports() {
        let name = format!("{}/{}", import_type.module(), import_type.name());
        let pf = match provided_fns.get(&name) {
            Some(rc_fn) => rc_fn.clone(),
            _ => panic!("don't have function {}", name),
        };
        let f = wasmtime::Func::new(&store, coerce_fn(import_type.ty()), pf);
        let x = wasmtime::Extern::Func(wasmtime::HostRef::new(f));
        imps.push(x);
    }

    inst.with(|i| {
        *i.borrow_mut() = Some(wasmtime::Instance::new(&store, &module, &imps).unwrap());
    });

    inst_call("helloworld", &[]);
}
