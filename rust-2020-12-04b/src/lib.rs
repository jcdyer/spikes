
use serde_json_old::Value as OldValue;
use serde_json::Value;

fn from_old(val: OldValue) -> Value {
    match val {
        OldValue::Null => Value::Null,
        OldValue::Bool(b) => Value::Bool(b),
        OldValue::String(s) => Value::String(s),
        OldValue::Array(a) => Value::Array(a.into_iter().map(from_old).collect()),
        OldValue::Object(o) => Value::Object(o.into_iter().map(|(k, v)| (k, from_old(v))).collect()),
        OldValue::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap().into()
            } else if n.is_u64() {
                n.as_u64().unwrap().into()
            } else {
                n.as_f64().unwrap().into()
            }
        }
    }
}

