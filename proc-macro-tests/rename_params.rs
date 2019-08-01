#![feature(param_attrs)]
use params_attribute_example::rename_params;

#[rename_params]
fn hello(a: i32, #[val] b: i32) {

}