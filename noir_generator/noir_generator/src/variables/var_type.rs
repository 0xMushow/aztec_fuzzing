use std::vec;

use crate::random;
use super::operator::Operator;

const MAX_COMPOSITE_DEPTH: usize = 10;
pub const MAX_COMPOSITE_SIZE: usize = 10;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum VarType {
    Field,
    u8,
    u16,
    u32,
    u64,
    u127,
    i8,
    i16,
    i32,
    i64,
    i127,
    bool,
    str(usize),
    Array(Box<VarType>, usize),
    Slice(Box<VarType>),
    // Vec(Box<VarType>),
    tup(Vec<Box<VarType>>),
}

impl Eq for VarType {}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            
            VarType::Field | VarType::u8 | VarType::u16 | VarType::u32 | VarType::u64 | VarType::u127 | VarType::i8 | VarType::i16 
            | VarType::i32 | VarType::i64 | VarType::i127 | VarType::bool => write!(f, "{:?}", self),
            VarType::str(size) => write!(f, "str<{}>", size),

            VarType::Array(type_param, size) => write!(f, "[{}; {}]", type_param, size),
            VarType::Slice(type_param) => write!(f, "[{}]", type_param),
            VarType::tup(vec_type_param) => {
                write!(f, "(")?;
                for (i, type_param) in vec_type_param.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", type_param)?;
                }
                write!(f, ")")
            },
        }
    }
}


pub fn basic_types() -> Vec<VarType> {
    vec![
        VarType::Field,
        VarType::u8,
        VarType::u16,
        VarType::u32,
        VarType::u64,
        VarType::u127,
        VarType::i8,
        VarType::i16,
        VarType::i32,
        VarType::i64,
        VarType::i127,
        VarType::bool,
    ]
}

pub fn random_basic_type() -> VarType {
    match random::gen_range(0, 11) {
        0 => VarType::Field,
        1 => VarType::u8,
        2 => VarType::u16,
        3 => VarType::u32,
        4 => VarType::u64,
        5 => VarType::u127,
        6 => VarType::i8,
        7 => VarType::i16,
        8 => VarType::i32,
        9 => VarType::i64,
        10 => VarType::i127,
        11 => VarType::bool,
        _ => VarType::Field,
    }
}

pub fn random_type() -> VarType {
    match random::gen_range(0, 15) {
        0 => VarType::Field,
        1 => VarType::u8,
        2 => VarType::u16,
        3 => VarType::u32,
        4 => VarType::u64,
        5 => VarType::u127,
        6 => VarType::i8,
        7 => VarType::i16,
        8 => VarType::i32,
        9 => VarType::i64,
        10 => VarType::i127,
        11 => VarType::bool,
        12 => VarType::str(random::gen_range(0, MAX_COMPOSITE_SIZE)),
        13 => VarType::Array(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH)), random::gen_range(0, MAX_COMPOSITE_SIZE)),
        14 => VarType::Slice(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH))),
        15 => {
            let size = random::gen_range(0, MAX_COMPOSITE_SIZE);
            let mut vec_tup = Vec::with_capacity(size);
            for _ in 0..size {
                vec_tup.push(Box::new(random_type_with_depth(MAX_COMPOSITE_DEPTH)));
            }
            VarType::tup(vec_tup)
        },
        _ => VarType::Field,
    }
}

fn random_type_with_depth(depth: usize) -> VarType {
    if depth == 0 {
        random::choose_random_item_from_vec(&basic_types())
    } else {
        match random::gen_range(0, 15) {
            0 => VarType::Field,
            1 => VarType::u8,
            2 => VarType::u16,
            3 => VarType::u32,
            4 => VarType::u64,
            5 => VarType::u127,
            6 => VarType::i8,
            7 => VarType::i16,
            8 => VarType::i32,
            9 => VarType::i64,
            10 => VarType::i127,
            11 => VarType::bool,
            12 => VarType::str(random::gen_range(0, MAX_COMPOSITE_SIZE)),
            13 => VarType::Array(Box::new(random_type_with_depth(depth -1)), random::gen_range(0, MAX_COMPOSITE_SIZE)),
            14 => VarType::Slice(Box::new(random_type_with_depth(depth -1))),
            15 => {
                let size = random::gen_range(0, MAX_COMPOSITE_SIZE);
                let mut vec_tup = Vec::with_capacity(size);
                for _ in 0..size {
                    vec_tup.push(Box::new(random_type_with_depth(depth -1)));
                }
                VarType::tup(vec_tup)
            },
            _ => VarType::Field,
        }
    }
}

pub fn supported_arithmetic_operator_by_type(var_type: VarType) -> Vec<Operator> {
    match var_type {
        VarType::Field | VarType::i8 | VarType::i16 | VarType::i32 | VarType::i64 | VarType::i127
            => vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide],
        VarType::u8 | VarType::u16 | VarType::u32 | VarType::u64 | VarType::u127
            => vec![Operator::Add, Operator::Subtract, Operator::Multiply, Operator::Divide, Operator::Xor, Operator::And, Operator::Or, Operator::Lshift, Operator::Rshift],
        VarType::bool => vec![Operator::Equal, Operator::NotEqual, Operator::Or, Operator::And],
        _ => vec![], // Handle unknown types
    }
}

pub fn supported_comparator_operator_by_type(var_type: VarType) -> Vec<Operator> {
    match var_type {
        VarType::Field | VarType::u8 | VarType::u16 | VarType::u32 | VarType::u64 | VarType::u127 | VarType::i8 | VarType::i16 | VarType::i32 | VarType::i64 | VarType::i127
            => vec![Operator::Equal, Operator::NotEqual, Operator::Lesser, Operator::Greater, Operator::LesserOrEqual, Operator::GreaterOrEqual],
        VarType::bool => vec![Operator::Equal, Operator::NotEqual, Operator::Or, Operator::And],
        VarType::str(_) | VarType::Slice(_) | VarType::Array(_,_) => vec![Operator::Equal, Operator::NotEqual],
        _ => vec![], // Handle unknown types
    }
}