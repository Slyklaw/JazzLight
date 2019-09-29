use super::*;
use crate::*;
use pgc::*;
use value::*;

pub fn pop(this: Value, _: &[Value]) -> Result<Value, Value> {
    match this {
        Value::Object(object) => match &object.kind {
            ObjectKind::Array(array) => return Ok(array.get_mut().pop().unwrap_or(Value::Null)),
            _ => Err(Value::Null),
        },
        _ => Err(Value::Null),
    }
}

pub fn array_object() {
    let state = STATE.lock();
    // we need object to be rooted since it may be deleted by GC.
    let object = Rooted::new(Object {
        kind: ObjectKind::Ordinary,
        proto: None,
        properties: Gc::new(vec![]),
    });

    object.get_mut().set_property(
        Value::String(Gc::new("pop".to_owned())),
        new_builtin_fn(pop as usize, 0),
    );
    state.get_mut().static_variables.insert(
        Value::String(Gc::new("Array".to_owned())),
        Value::Object(object.inner()), // now 'Array' object unrooted,but since global state is rooted it's fine.
    );
}
