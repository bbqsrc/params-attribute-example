#![feature(param_attrs)]
use params_attribute_example::rename_params;

#[rename_params(send_help)]
fn hello(#[angery(true)] a: i32, b: i32) {}
