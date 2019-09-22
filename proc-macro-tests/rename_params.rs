#![feature(param_attrs)]
use params_attribute_example::rename_params;

#[rename_params(send_help)]
fn hello(#[angery(true)] a: i32, #[a2] b: i32, #[what = "how"] c: u32) {}

#[rename_params(send_help)]
impl Foo {
  fn hello(#[angery(true)] a: i32, #[a2] b: i32, #[what = "how"] c: u32) {}
  fn hello2(#[a1] #[a2] a: i32, #[what = "how"] b: i32, #[angery(true)] c: u32) {}
  fn hello_self(#[a1] #[a2] &self, #[a1] #[a2] a: i32, #[what = "how"] b: i32, #[angery(true)]c: u32) {}
}