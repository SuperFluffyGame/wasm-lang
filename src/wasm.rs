use crate::op_code::ValueType;

pub struct Func {
    name: String,
    params: Vec<ValueType>,
    result: Vec<ValueType>,
}
