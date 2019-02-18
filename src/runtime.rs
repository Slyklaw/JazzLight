use rand::Rng;
use waffle::builtins::init_builtins;
use waffle::builtins::read_line;
use waffle::value::{Float, FuncKind, Function, Value};
use waffle::VirtualMachine;

pub fn typename(vm: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    assert!(args.get(0).is_some());
    match &args[0]
    {
        Value::Int(_) => Value::Str(String::from("int")),
        Value::Float(_) => Value::Str(String::from("float")),
        Value::Bool(_) => Value::Str(String::from("bool")),
        Value::ArrayRef(_) => Value::Str(String::from("array")),
        Value::FuncRef(_) => Value::Str(String::from("function")),
        Value::ObjectRef(id) =>
        {
            let obj = vm.pool.get_object(*id);
            let key = Value::Str(String::from("__typename__"));
            if obj.contains(&key).0
            {
                obj.find(&key).unwrap().clone()
            }
            else
            {
                Value::Str(String::from("object"))
            }
        }
        Value::Str(_) => Value::Str(String::from("string")),
        Value::Null => Value::Str(String::from("null")),
    }
}

pub fn error(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    println!("Error: {}", args[0].as_str());
    std::process::exit(-1);
}

pub fn rand_int(_: &mut VirtualMachine, _: Vec<Value>) -> Value
{
    Value::Int(rand::random())
}

pub fn rand_float(_: &mut VirtualMachine, _: Vec<Value>) -> Value
{
    Value::Float(Float::new(rand::random()))
}

pub fn rand_range(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    Value::Int(rand::thread_rng().gen_range(args[0].as_int(), args[1].as_int()))
}

pub fn string_trim(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    assert!(args[0].is_str());
    Value::Str(args[0].as_str().trim().to_owned())
}

pub fn len(vm: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    match &args[0]
    {
        Value::Str(s) => Value::Int(s.len() as i64),
        Value::ArrayRef(arr) =>
        {
            let arr = vm.pool.get_array(*arr);
            Value::Int(arr.elements.len() as i64)
        }
        Value::ObjectRef(obj) =>
        {
            let obj = vm.pool.get_object(*obj);
            Value::Int(obj.table.len() as i64)
        }
        _ => panic!("Can't get len on {:?}", args[0]),
    }
}

pub fn clrscr(_: &mut VirtualMachine, _: Vec<Value>) -> Value
{
    let term = crossterm_terminal::terminal();
    term.clear(crossterm_terminal::ClearType::All).unwrap();
    Value::Null
}

pub fn putchar(_: &mut VirtualMachine,args: Vec<Value>) -> Value {
    let arg = &args[0];
    match arg {
        Value::Str(s) => print!("{}",s.chars().nth(0).unwrap()),
        Value::Int(i) => print!("{}",std::char::from_u32(*i as u32).unwrap()),
        _ => unimplemented!(),
    }
    Value::Null
}

pub fn sleep(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    let time = args[0].as_int();
    std::thread::sleep(std::time::Duration::from_millis(time as u64));
    Value::Null
}

pub fn char_from_int(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    match &args[0]
    {
        Value::Int(i) => Value::Str(std::char::from_u32(*i as u32).unwrap().to_string()),
        _ => unimplemented!(),
    }
}

pub fn char_to_int(_: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    match &args[0]
    {
        Value::Str(s) => Value::Int(s.chars().nth(0).unwrap() as u32 as i64),
        _ => unimplemented!(),
    }
}

pub fn clone(vm: &mut VirtualMachine, args: Vec<Value>) -> Value
{
    match &args[0]
    {
        Value::Int(i) => Value::Int(*i),
        Value::Float(f) => Value::Float(*f),
        Value::Str(s) => Value::Str(s.clone()),
        Value::ArrayRef(id) =>
        {
            let arr = vm.pool.get_array(*id).clone();
            Value::ArrayRef(vm.pool.add_array(arr, false))
        }
        Value::ObjectRef(id) =>
        {
            let obj = vm.pool.get_object(*id).clone();
            Value::ObjectRef(vm.pool.add_object(obj, false))
        }
        Value::Null => Value::Null,
        Value::Bool(b) => Value::Bool(*b),
        _ => unimplemented!(),
    }
}

use fxhash::FxHashMap;

pub fn init(vm: &mut VirtualMachine) -> FxHashMap<&'static str, usize>
{
    macro_rules! register {
        ($map:expr => $name:ident $argc:expr) => {
            let func = Function {
                addr: FuncKind::Native($name as *const u8),
                is_native: true,
                nargs: $argc,
                nlocals: 0,
            };

            let id = vm.pool.add_func(func);
            let name: &str = &format!("builtin_{}",stringify!($name));
            $map.insert(unsafe {std::mem::transmute::<&str,&'static str>(name)},id);

        };
        ($map:expr => ($($name:ident $argc:expr),*)) => {
            $(
                register!($map => $name $argc);
            )*
        }
    }
    macro_rules! simply_register {($map:expr => $name:ident) => {
        let name: &str = &format!("builtin_{}", stringify!($name));
        let fid = $map.get(name).unwrap();
        $map.insert(stringify!($name), *fid);
    };
    }

    let mut map = init_builtins(vm);

    register!(map => (
            len 1,
            rand_range 2,
            rand_int 0,
            rand_float 0,
            error 1,
            read_line 0,
            string_trim 1,
            typename 1,
            char_to_int 1,
            char_from_int 1,
            clrscr 0,
            sleep 1,
            clone 1,
            putchar 1
        )
    );

    simply_register!(map => rand_range);
    simply_register!(map => rand_int);
    simply_register!(map => rand_float);
    simply_register!(map => error);
    simply_register!(map => read_line);
    simply_register!(map => string_trim);
    simply_register!(map => len);
    simply_register!(map => typename);
    simply_register!(map => char_to_int);
    simply_register!(map => clrscr);
    simply_register!(map => sleep);
    simply_register!(map => char_from_int);
    simply_register!(map => clone);
    simply_register!(map => putchar);

    map
}
