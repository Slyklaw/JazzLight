use crate::module::Module;
use crate::value::*;
use crate::vm::{FIELDS, VM};
use crate::P;

pub extern "C" fn load(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    use libloading as lib;
    if val_is_str(&args[0]) {
        let path = val_str(&args[0]);
        let symbol_name = val_str(&args[1]);
        let nargs = val_int(&args[2]);
        let lib = lib::Library::new(&path).unwrap();

        unsafe {
            let func: lib::Symbol<*mut u8> = lib.get(symbol_name.as_bytes()).unwrap();
            let func = Function {
                var: FuncVar::Native((*func) as *const u8),
                nargs: nargs as i32,
                jit: false,
                module: P(Module::new(&path)),
                env: P(Value::Null),
                yield_point: 0,
            };
            P(Value::Func(P(func)))
        }
    } else {
        panic!("String expected");
    }
}

pub extern "C" fn os_string(_: &mut VM, _args: Vec<P<Value>>) -> P<Value> {
    P(Value::Str(std::env::consts::OS.to_owned()))
}

pub fn val_string(vm: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let mut buff = String::new();
    for val in args.iter() {
        match val.borrow() {
            Value::Int(i) => buff.push_str(&i.to_string()),
            Value::Int32(i) => buff.push_str(&i.to_string()),
            Value::Float(f) => buff.push_str(&f.to_string()),
            Value::Func(_) => buff.push_str("<function>"),
            Value::Bool(b) => buff.push_str(&b.to_string()),
            Value::Null => buff.push_str("null"),
            Value::Str(s) => buff.push_str(s),
            Value::Object(obj) => {
                let obj: &Object = obj.borrow();
                buff.push_str("{ ");
                for (idx, entry) in obj.entries.iter().enumerate() {
                    let name = FIELDS.borrow().get(&(entry.hash as u64)).unwrap();
                    buff.push_str(&format!("{} => ", name));
                    let entry = entry.borrow();
                    let val = entry.val.clone();
                    let s = val_string(vm, vec![val]);
                    if let Value::Str(s) = s.borrow() {
                        buff.push_str(s);
                    }
                    if idx != obj.entries.len() - 1 {
                        buff.push_str(", ");
                    }
                }
                buff.push_str(" }");
            }
            Value::Array(values) => {
                let arr = values.borrow();
                buff.push_str("[");
                for (idx, val) in arr.iter().enumerate() {
                    let s = val_string(vm, vec![val.clone()]);
                    if let Value::Str(s) = s.borrow() {
                        buff.push_str(s);
                        if idx != arr.len() - 1 {
                            buff.push_str(", ");
                        }
                    }
                }
                buff.push_str("]");
            }
            Value::Extern(ptr, name) => buff.push_str(&format!("<{}> at {:?}", name, ptr)),
        }
    }

    P(Value::Str(buff))
}

pub extern "C" fn print(vm: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if let Value::Str(val) = val_string(vm, args).borrow() {
        print!("{}", val);
    }

    P(Value::Null)
}
#[no_mangle]
pub extern "C" fn loader_loadmodule(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let name = if let Value::Str(s) = args[0].borrow() {
        s.to_owned()
    } else {
        println!("{:?}", args[0]);
        unreachable!();
    };
    //let vthis = args[1].clone();

    use crate::module::*;
    use std::fs::File;
    use std::io::Read;

    let mut reader = Reader {
        code: vec![],
        pc: 0,
    };

    let env: Option<&'static str> = option_env!("JAZZ_PATH");

    let path = if env.is_some() {
        let path = format!("{}{}", env.unwrap(), name);
        let p = std::path::Path::new(&path);
        if p.exists() {
            path
        } else {
            let path = format!("{}{}.j", env.unwrap(), name);
            let p = if std::path::Path::new(&path).exists() {
                path
            } else {
                name
            };
            p
        }
    } else {
        if std::path::Path::new(&name).exists() {
            name
        } else {
            let ps = format!("{}.j", &name);
            let p = std::path::Path::new(&ps);
            if p.exists() {
                ps
            } else {
                panic!("File not found");
            }
        }
    };
    let mut f = File::open(&path).unwrap();
    f.read_to_end(&mut reader.code).unwrap();

    let mut module = read_module(reader, &path);

    let mut vm = VM::new();
    vm.builtins = crate::vm::VM_THREAD.builtins.clone();
    vm.code = module.code.clone();
    vm.interp(&mut module);

    let exports = module.exports.clone();

    return exports;
}

pub extern "C" fn array(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    P(Value::Array(P(args)))
}

pub extern "C" fn alen(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow();

        return P(Value::Int(array.len() as i64));
    } else {
        panic!("Array expected");
    }
}

pub extern "C" fn string_from_bytes(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow();
        let mut bytes = vec![];
        for val in array.iter() {
            let int = val_int(val);
            bytes.push(int as u8);
        }
        return P(Value::Str(String::from_utf8(bytes).unwrap()));
    } else {
        panic!("Array expected");
    }
}

pub extern "C" fn string_bytes(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_str(&args[0]) {
        let string = val_str(&args[0]);
        let mut bytes = vec![];
        for byte in string.as_bytes().iter() {
            bytes.push(P(Value::Int(*byte as i64)));
        }

        P(Value::Array(P(bytes)))
    } else {
        panic!("String expected")
    }
}

pub extern "C" fn apush(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow_mut();
        let val = args[1].clone();
        array.push(val);
    }

    P(Value::Null)
}

pub extern "C" fn apop(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow_mut();
        array.pop().unwrap_or(P(Value::Null))
    } else {
        P(Value::Null)
    }
}

pub extern "C" fn aset(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow_mut();
        let key = val_int(&args[1]);
        let val = args[2].clone();
        array[key as usize] = val;
    }
    // Throw error if val not array?
    P(Value::Null)
}

pub extern "C" fn aget(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_array(&args[0]) {
        let array_p = val_array(&args[0]);
        let array = array_p.borrow();
        let key = val_int(&args[1]);
        return array.get(key as usize).unwrap_or(&P(Value::Null)).clone();
    }
    P(Value::Null)
}
use crate::Cell;
lazy_static::lazy_static!(
    pub static ref THREADS: Cell<fnv::FnvHashMap<i32,Cell<std::thread::JoinHandle<P<Value>>>>> = Cell::new(fnv::FnvHashMap::default());
);

pub extern "C" fn thread_spawn(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    if val_is_func(&args[0]) {
        use crate::vm::callex;
        let val = args[0].clone();

        let thread = std::thread::spawn(|| callex(P(Value::Null), val, vec![]));
        let id = thread.thread().id();
        let id: u64 = unsafe { std::mem::transmute(id) };

        THREADS.borrow_mut().insert(id as i32, Cell::new(thread));

        return P(Value::Int32(id as i32));
    }
    P(Value::Null)
}

pub extern "C" fn thread_join(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let val = args[0].borrow();
    match &val {
        Value::Int32(idx) => {
            let thread = THREADS
                .borrow()
                .get(idx)
                .expect("Thread not found")
                .direct();

            thread.join().unwrap()
        }
        _ => P(Value::Null),
    }
}

macro_rules! new_builtin {
    ($vm: expr,$f: ident) => {
        let f = Function {
            nargs: -1,
            var: FuncVar::Native($f as *const u8),
            module: P(Module::new("__0")),
            env: P(Value::Null),
            jit: false,
            yield_point: 0,
        };
        $vm.builtins.push(P(Value::Func(P(f))));
    };
}

pub extern "C" fn file_size(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let file = val_object(&args[0]);
    let hash = crate::fields::hash_str;

    let h_file = hash("__handle");
    let field = file.find(h_file).unwrap();
    if let Value::Str(fname) = field.borrow() {
        let f = std::fs::File::open(fname).unwrap().metadata().unwrap();
        return P(Value::Int(f.len() as i64));
    } else {
        panic!("File not found?");
    };
}

pub extern "C" fn file_write(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let file = val_object(&args[0]);
    let array = if val_is_array(&args[1]) {
        args[1].clone()
    } else {
        panic!("Array expected");
    };

    let p = val_int(&args[2]);
    let l = val_int(&args[3]);

    let hash = crate::fields::hash_str;

    let h_file = hash("__handle");

    use std::io::Seek;
    use std::io::Write;
    let field = file.find(h_file).unwrap();
    let buf = val_array(&array);
    let mut bytes = vec![];
    for i in 0..l as usize {
        let x = buf.get(i).unwrap();
        if let Value::Int(i) = x.borrow() {
            bytes.push(*i as u8);
        }
        if let Value::Int32(i) = x.borrow() {
            bytes.push(*i as u8);
        }
    }
    if let Value::Str(fname) = field.borrow() {
        let mut f = std::fs::OpenOptions::new().write(true).open(fname).unwrap();
        f.seek(std::io::SeekFrom::Start(p as _)).unwrap();
        f.write_all(&mut bytes).unwrap();
    } else {
        panic!("File not found?");
    };

    P(Value::Null)
}

pub extern "C" fn file_read(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let file = val_object(&args[0]);
    let array = if val_is_array(&args[1]) {
        args[1].clone()
    } else {
        panic!("Array expected");
    };

    let p = val_int(&args[2]);
    let l = val_int(&args[3]);

    let hash = crate::fields::hash_str;

    let h_file = hash("__handle");

    use std::fs::File;
    use std::io::Read;
    use std::io::Seek;
    let field = file.find(h_file).unwrap();
    let mut buf = vec![0u8; l as usize];
    if let Value::Str(fname) = field.borrow() {
        let mut f = File::open(fname).unwrap();
        f.seek(std::io::SeekFrom::Start(p as _)).unwrap();
        f.read_exact(&mut buf).unwrap();
    } else {
        panic!("File not found?");
    };

    let arr_p = val_array(&array);
    let arr = arr_p.borrow_mut();
    for byte in buf.iter() {
        arr.push(P(Value::Int(*byte as i64)));
    }
    P(Value::Null)
}

pub fn register_builtins(vm: &mut VM) {
    new_builtin!(vm, load);
    new_builtin!(vm, val_string);
    new_builtin!(vm, print);
    new_builtin!(vm, array);
    new_builtin!(vm, alen);
    new_builtin!(vm, apush);
    new_builtin!(vm, apop);
    new_builtin!(vm, aset);
    new_builtin!(vm, aget);
    new_builtin!(vm, os_string);
    new_builtin!(vm, thread_spawn);
    new_builtin!(vm, thread_join);
    new_builtin!(vm, loader_loadmodule);
    new_builtin!(vm, string_bytes);
    new_builtin!(vm, string_from_bytes);
    new_builtin!(vm, file);
    new_builtin!(vm, file_read);
    new_builtin!(vm, file_write);
    new_builtin!(vm, file_size);
}

pub extern "C" fn file(_: &mut VM, args: Vec<P<Value>>) -> P<Value> {
    let mut obj = Object { entries: vec![] };
    let vname = val_str(&args[0]);

    macro_rules! new_field {
        ($name: expr,$val: expr) => {{
            let hash = crate::fields::hash_str($name);
            obj.insert(hash, P($val));
        }};
    }
    new_field!("__handle", Value::Str(vname.to_owned()));
    P(Value::Object(P(obj)))
}

pub fn loader(module: &P<Module>) -> P<Value> {
    let mut obj = Object { entries: vec![] };

    let f = Function {
        var: FuncVar::Native(loader_loadmodule as i64 as *const u8),
        env: P(Value::Array(P(vec![]))),
        module: module.clone(),
        nargs: 1,
        jit: false,
        yield_point: 0,
    };

    obj.insert(crate::fields::hash_str("loadmodule"), P(Value::Func(P(f))));

    P(Value::Object(P(obj)))
}
