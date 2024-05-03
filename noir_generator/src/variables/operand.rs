use crate::variables::variable::Variable;
use crate::variables::operation::Operation;
use super::value::Value;
use super::var_type::VarType;

#[derive(Clone)]
pub(crate) enum Operand {
    Variable(Variable),
    Operation(Box<Operation>),
    Value(Value, VarType),
}
