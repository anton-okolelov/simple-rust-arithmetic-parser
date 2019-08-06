extern crate libc;

use crate::expression::Expression;
use crate::expression::Expression::*;
use crate::parser::*;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn parse_arithmetic(s: *const c_char) -> *mut ExpressionFfi {
    unsafe {
        // todo: error handling
        let rust_string = CStr::from_ptr(s).to_str().unwrap();
        parse(rust_string).unwrap().convert_to_c()
    }
}

impl Expression {
    fn convert_to_c(&self) -> *mut ExpressionFfi {
        let expression_data = match self {
            Value(value) => ExpressionData { value: *value },
            Add(left, right)
            | Subtract(left, right)
            | Multiply(left, right)
            | Divide(left, right) => ExpressionData {
                pair_operands: PairOperands {
                    left: left.convert_to_c(),
                    right: right.convert_to_c(),
                },
            },
            UnaryMinus(operand) => ExpressionData {
                single_operand: operand.convert_to_c(),
            },
        };

        let expression_ffi = match self {
            Add(_, _) => ExpressionFfi {
                expression_type: ExpressionType::Add,
                data: expression_data,
            },
            Subtract(_, _) => ExpressionFfi {
                expression_type: ExpressionType::Subtract,
                data: expression_data,
            },
            Multiply(_, _) => ExpressionFfi {
                expression_type: ExpressionType::Multiply,
                data: expression_data,
            },
            Divide(_, _) => ExpressionFfi {
                expression_type: ExpressionType::Multiply,
                data: expression_data,
            },
            UnaryMinus(_) => ExpressionFfi {
                expression_type: ExpressionType::UnaryMinus,
                data: expression_data,
            },
            Value(_) => ExpressionFfi {
                expression_type: ExpressionType::Value,
                data: expression_data,
            },
        };
        Box::into_raw(Box::new(expression_ffi))
    }
}


#[repr(C)]
pub struct ExpressionFfi {
    expression_type: ExpressionType,
    data: ExpressionData,
}

#[repr(u8)]
pub enum ExpressionType {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    Divide = 3,
    UnaryMinus = 4,
    Value = 5,
}

#[repr(C)]
pub union ExpressionData {
    pair_operands: PairOperands,
    single_operand: *mut ExpressionFfi,
    value: i64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairOperands {
    left: *mut ExpressionFfi,
    right: *mut ExpressionFfi,
}

