use std::any::Any;
use std::fmt;

/// Represents primitive types that are supported for conversion into a BTreeMap that can support
/// heterogeneous values. Inspired by `serde_json::Value`s.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Num(Num),
    String(String),
    Array(Vec<Value>),
    // TODO: Map
}

/// Represents the numeric primitive types that are supported for conversion.
#[derive(Debug, Clone)]
pub enum Num {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    /// Given a genericized input type, encapsulate it as a Value that can be used in a map
    /// container type when converting to and from a struct.
    pub fn new<T: Any>(value: T) -> Value {
        let any_val = &value as &dyn Any;
        if let Some(val) = any_val.downcast_ref::<bool>() {
            Value::Bool(*val)
        } else if let Some(val) = any_val.downcast_ref::<i32>() {
            Value::Num(Num::I32(*val))
        } else if let Some(val) = any_val.downcast_ref::<i64>() {
            Value::Num(Num::I64(*val))
        } else if let Some(val) = any_val.downcast_ref::<u32>() {
            Value::Num(Num::U32(*val))
        } else if let Some(val) = any_val.downcast_ref::<u64>() {
            Value::Num(Num::U64(*val))
        } else if let Some(val) = any_val.downcast_ref::<f32>() {
            Value::Num(Num::F32(*val))
        } else if let Some(val) = any_val.downcast_ref::<f64>() {
            Value::Num(Num::F64(*val))
        } else if let Some(val) = any_val.downcast_ref::<&'static str>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<String>() {
            Value::String(val.to_string())
        } else if let Some(val) = any_val.downcast_ref::<Vec<Value>>() {
            Value::Array(val.to_vec())
        } else {
            Value::Null
        }
    }

    pub fn bool(&self) -> Option<bool> {
        if let Value::Bool(val) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn i32(&self) -> Option<i32> {
        if let Value::Num(Num::I32(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn i64(&self) -> Option<i64> {
        if let Value::Num(Num::I64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn u32(&self) -> Option<u32> {
        if let Value::Num(Num::U32(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn u64(&self) -> Option<u64> {
        if let Value::Num(Num::U64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn f32(&self) -> Option<f32> {
        if let Value::Num(Num::F32(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn f64(&self) -> Option<f64> {
        if let Value::Num(Num::F64(val)) = self {
            Some(*val)
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<String> {
        if let Value::String(string) = self {
            Some(string.to_string())
        } else {
            None
        }
    }
}
