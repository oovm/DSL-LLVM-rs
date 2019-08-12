extern crate inkwell;

use inkwell::context::Context;


#[test]
fn create_str() {
    let context = Context::create();
    let vec = context.const_string("new_string", false);
    let llvm_str = vec.print_to_string();
    assert_eq!(llvm_str.to_string(), "[10 x i8] c\"new_string\"");
    let c_str = vec.get_string_constant();
    assert_eq!(c_str.to_str().unwrap(), "new_string");
}


#[test]
fn create_bool() {
    let context = Context::create();
    // bool is a kind of Int in llvm
    let bool_type = context.bool_type();
    let bool_value = bool_type.const_int(1, false);
    // which bit width = 1
    assert_eq!(bool_type.get_bit_width(), 1);
}


#[test]
fn create_i64() {
    let context = Context::create();
    let i64_type = context.i64_type();
    let i64_value = i64_type.const_int(42, false);
    assert!(i64_value.is_const());
    assert_eq!(i64_value.get_sign_extended_constant().unwrap(), 42);
}

#[test]
fn create_f64() {
    let context = Context::create();
    let f64_type = context.f64_type();
    let f64_value = f64_type.const_float(3.14);
    assert!(f64_value.is_const());
    let c = f64_value.get_constant().unwrap();
    assert_eq!(c, (3.14, false));
}

#[test]
fn create_struct() {
    let context = Context::create();
    let f32_type = context.f32_type();
    let i16_type = context.i16_type();
    let f32_one = f32_type.const_float(1.);
    let i16_two = i16_type.const_int(2, false);
    let const_struct = context.const_struct(&[i16_two.into(), f32_one.into()], true);

    assert_eq!(const_struct.get_type().get_field_types(), &[i16_type.into(), f32_type.into()]);
}
