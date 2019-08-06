<?php

ini_set("display_errors", "on");
error_reporting(E_ALL);

$cdef = \FFI::cdef(file_get_contents("target/testffi.h"), "target/debug/libexpr_parser.so");
$expression = $cdef->parse_arithmetic("-6-(4+5)+(5+5)*(4-4)");

printExpression($expression);

class ExpressionKind {
    const Add = 0;
    const Subtract = 1;
    const Multiply = 2;
    const Divide = 3;
    const UnaryMinus = 4;
    const Value = 5;
}

function printExpression($expression) {
    switch ($expression->expression_type) {
        case ExpressionKind::Add:
        case ExpressionKind::Subtract:
        case ExpressionKind::Multiply:
        case ExpressionKind::Divide:
            $operations = ["+", "-", "*", "/"];
            print "(";
            printExpression($expression->data->pair_operands->left);
            print $operations[$expression->expression_type];
            printExpression($expression->data->pair_operands->right);
            print ")";
            break;
        case ExpressionKind::UnaryMinus:
            print "-";
            printExpression($expression->data->single_operand);
            break;
        case ExpressionKind::Value:
            print $expression->data->value;
            break;
    }
}



