use crate::runtime::MemAddr;

pub type VsResult<T> = Result<T, OperationNotDefined>;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Decimal(f64),
    Natural(u64),
    Signed(i64),
    String(String),
    Addr(MemAddr),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => "nil".fmt(f),
            Value::Boolean(inner) => inner.fmt(f),
            Value::Decimal(inner) => inner.fmt(f),
            Value::Natural(inner) => inner.fmt(f),
            Value::Signed(inner) => inner.fmt(f),
            Value::Addr(inner) => inner.fmt(f),
            Value::String(inner) => inner.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperationNotDefined;

fn cast_to_common(lhs: Value, rhs: Value) -> VsResult<(Value, Value)> {
    match (&lhs, &rhs) {
        (Value::Nil, _) => Ok((lhs, rhs)),
        (_, Value::Nil) => Ok((lhs, rhs)),
        (Value::Boolean(_), Value::Boolean(_)) => Ok((lhs, rhs)),
        (Value::Natural(_), Value::Natural(_)) => Ok((lhs, rhs)),
        (Value::Decimal(_), Value::Decimal(_)) => Ok((lhs, rhs)),
        (Value::Signed(_), Value::Signed(_)) => Ok((lhs, rhs)),
        (Value::Signed(_), Value::Natural(rhs)) => Ok((lhs, Value::Signed(*rhs as i64))),
        (Value::Natural(lhs), Value::Signed(_)) => Ok((Value::Signed(*lhs as i64), rhs)),
        (Value::Natural(lhs), Value::Decimal(_)) => Ok((Value::Decimal(*lhs as f64), rhs)),
        (Value::Decimal(_), Value::Natural(rhs)) => Ok((lhs, Value::Decimal(*rhs as f64))),
        (Value::Signed(lhs), Value::Decimal(_)) => Ok((Value::Decimal(*lhs as f64), rhs)),
        (Value::Decimal(_), Value::Signed(rhs)) => Ok((lhs, Value::Decimal(*rhs as f64))),
        _ => Err(OperationNotDefined),
    }
}
fn inner_equal(lhs: Value, rhs: Value) -> VsResult<bool> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(false),
        (_, Value::Nil) => Ok(false),
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(lhs == rhs),
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(lhs == rhs),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(lhs == rhs),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(lhs == rhs),
        _ => Err(OperationNotDefined),
    }
}

fn inner_less(lhs: Value, rhs: Value) -> VsResult<bool> {
    match cast_to_common(lhs, rhs)? {
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(lhs < rhs),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(lhs < rhs),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(lhs < rhs),
        _ => Err(OperationNotDefined),
    }
}

fn inner_less_or_equal(lhs: Value, rhs: Value) -> VsResult<bool> {
    match cast_to_common(lhs, rhs)? {
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(lhs <= rhs),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(lhs <= rhs),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(lhs <= rhs),
        _ => Err(OperationNotDefined),
    }
}

fn inner_greater(lhs: Value, rhs: Value) -> VsResult<bool> {
    match cast_to_common(lhs, rhs)? {
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(lhs > rhs),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(lhs > rhs),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(lhs > rhs),
        _ => Err(OperationNotDefined),
    }
}

fn inner_greater_or_equal(lhs: Value, rhs: Value) -> VsResult<bool> {
    match cast_to_common(lhs, rhs)? {
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(lhs >= rhs),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(lhs >= rhs),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(lhs >= rhs),
        _ => Err(OperationNotDefined),
    }
}

pub fn equal(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_equal(lhs, rhs).map(Value::Boolean)
}

pub fn not_equal(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_equal(lhs, rhs).map(std::ops::Not::not).map(Value::Boolean)
}

pub fn less(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_less(lhs, rhs).map(Value::Boolean)
}

pub fn less_or_equal(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_less_or_equal(lhs, rhs).map(Value::Boolean)
}

pub fn greater(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_greater(lhs, rhs).map(Value::Boolean)
}

pub fn greater_or_equal(lhs: Value, rhs: Value) -> VsResult<Value> {
    inner_greater_or_equal(lhs, rhs).map(Value::Boolean)
}

pub fn add(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(Value::Natural(lhs.wrapping_add(rhs))),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(Value::Signed(lhs.wrapping_add(rhs))),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(Value::Decimal(lhs + rhs)),
        _ => Err(OperationNotDefined),
    }
}

pub fn sub(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(Value::Natural(lhs.wrapping_sub(rhs))),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(Value::Signed(lhs.wrapping_sub(rhs))),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(Value::Decimal(lhs - rhs)),
        _ => Err(OperationNotDefined),
    }
}

pub fn mul(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(Value::Natural(lhs.wrapping_mul(rhs))),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(Value::Signed(lhs.wrapping_mul(rhs))),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(Value::Decimal(lhs * rhs)),
        _ => Err(OperationNotDefined),
    }
}

pub fn div(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Natural(lhs), Value::Natural(rhs)) => Ok(Value::Natural(lhs / rhs)),
        (Value::Decimal(lhs), Value::Decimal(rhs)) => Ok(Value::Decimal(lhs / rhs)),
        (Value::Signed(lhs), Value::Signed(rhs)) => Ok(Value::Signed(lhs.wrapping_div(rhs))),
        _ => Err(OperationNotDefined),
    }
}

pub fn not(value: Value) -> VsResult<Value> {
    match value {
        Value::Nil => Ok(Value::Nil),
        Value::Boolean(value) => Ok(Value::Boolean(!value)),
        _ => Err(OperationNotDefined),
    }
}

pub fn and(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs && rhs)),
        _ => Err(OperationNotDefined),
    }
}

pub fn or(lhs: Value, rhs: Value) -> VsResult<Value> {
    match cast_to_common(lhs, rhs)? {
        (Value::Nil, _) => Ok(Value::Nil),
        (_, Value::Nil) => Ok(Value::Nil),
        (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(lhs || rhs)),
        _ => Err(OperationNotDefined),
    }
}

pub fn neg(value: Value) -> VsResult<Value> {
    match value {
        Value::Nil => Ok(Value::Nil),
        Value::Natural(value) => Ok(Value::Signed(-(value as i64))),
        Value::Decimal(value) => Ok(Value::Decimal(-value)),
        Value::Signed(value) => Ok(Value::Signed(-value)),
        _ => Err(OperationNotDefined),
    }
}
