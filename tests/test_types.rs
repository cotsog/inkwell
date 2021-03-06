extern crate inkwell;

use std::ffi::CString;

use self::inkwell::AddressSpace;
use self::inkwell::context::Context;
use self::inkwell::types::{FloatType, IntType, StructType, VoidType};

#[test]
fn test_struct_type() {
    let context = Context::create();
    let int = context.i8_type();
    let int_vector = int.vec_type(100);
    let float = context.f32_type();
    let float_array = float.array_type(3);
    let av_struct = context.struct_type(&[&int_vector, &float_array], false);

    assert!(!av_struct.is_packed());
    assert!(!av_struct.is_opaque());
    assert!(av_struct.is_sized());
    assert!(av_struct.get_name().is_none());
    assert_eq!(*av_struct.get_context(), context);
    assert_eq!(av_struct.count_fields(), 2);

    let field_1 = av_struct.get_field_type_at_index(0).unwrap();
    let field_2 = av_struct.get_field_type_at_index(1).unwrap();

    assert!(field_1.is_vector_type());
    assert!(field_2.is_array_type());
    assert!(av_struct.get_field_type_at_index(2).is_none());
    assert!(av_struct.get_field_type_at_index(200).is_none());
    assert_eq!(av_struct.get_field_types(), vec![field_1, field_2]);

    let av_struct = context.struct_type(&[&int_vector, &float_array], true);

    assert!(av_struct.is_packed());
    assert!(!av_struct.is_opaque());
    assert!(av_struct.is_sized());
    // REVIEW: Is there a way to name a non opaque struct?
    assert!(av_struct.get_name().is_none());
    assert_eq!(*av_struct.get_context(), context);
    assert_eq!(av_struct.count_fields(), 2);

    let field_1 = av_struct.get_field_type_at_index(0).unwrap();
    let field_2 = av_struct.get_field_type_at_index(1).unwrap();

    assert!(field_1.is_vector_type());
    assert!(field_2.is_array_type());
    assert!(av_struct.get_field_type_at_index(2).is_none());
    assert!(av_struct.get_field_type_at_index(200).is_none());
    assert_eq!(av_struct.get_field_types(), vec![field_1, field_2]);

    let opaque_struct = context.opaque_struct_type("opaque_struct");

    assert!(!opaque_struct.is_packed());
    assert!(opaque_struct.is_opaque());
    assert!(!opaque_struct.is_sized());
    assert_eq!(opaque_struct.get_name(), Some(&*CString::new("opaque_struct").unwrap()));
    assert_eq!(*opaque_struct.get_context(), context);
    assert_eq!(opaque_struct.count_fields(), 0);
    assert!(opaque_struct.get_field_type_at_index(0).is_none());
    assert!(opaque_struct.get_field_type_at_index(1).is_none());
    assert!(opaque_struct.get_field_type_at_index(2).is_none());
    assert!(opaque_struct.get_field_type_at_index(200).is_none());
    assert!(opaque_struct.get_field_types().is_empty());

    assert!(opaque_struct.set_body(&[&int_vector, &float_array], true));

    let no_longer_opaque_struct = opaque_struct;

    assert!(no_longer_opaque_struct.is_packed());
    assert!(!no_longer_opaque_struct.is_opaque());
    assert!(no_longer_opaque_struct.is_sized());
    assert_eq!(no_longer_opaque_struct.get_name(), Some(&*CString::new("opaque_struct").unwrap()));
    assert_eq!(*no_longer_opaque_struct.get_context(), context);
    assert_eq!(no_longer_opaque_struct.count_fields(), 2);

    let field_1 = no_longer_opaque_struct.get_field_type_at_index(0).unwrap();
    let field_2 = no_longer_opaque_struct.get_field_type_at_index(1).unwrap();

    assert!(field_1.is_vector_type());
    assert!(field_2.is_array_type());
    assert!(no_longer_opaque_struct.get_field_type_at_index(2).is_none());
    assert!(no_longer_opaque_struct.get_field_type_at_index(200).is_none());
    assert_eq!(no_longer_opaque_struct.get_field_types(), vec![field_1, field_2]);
}

#[test]
fn test_function_type() {
    let context = Context::create();
    let int = context.i8_type();
    let float = context.f32_type();
    let fn_type = int.fn_type(&[&int, &int, &float], false);

    assert!(!fn_type.is_var_arg());
    assert_eq!(*fn_type.get_context(), context);

    let param_types = fn_type.get_param_types();

    assert_eq!(param_types.len(), 3);
    assert_eq!(*param_types[0].as_int_type(), int);
    assert_eq!(*param_types[1].as_int_type(), int);
    assert_eq!(*param_types[2].as_float_type(), float);

    let fn_type = int.fn_type(&[&int, &float], true);

    assert!(fn_type.is_var_arg());
    assert_eq!(*fn_type.get_context(), context);
}

#[test]
fn test_sized_types() {
    let context = Context::get_global();
    let void_type = VoidType::void_type();
    let bool_type = IntType::bool_type();
    let i8_type = IntType::i8_type();
    let i16_type = IntType::i16_type();
    let i32_type = IntType::i32_type();
    let i64_type = IntType::i64_type();
    let i128_type = IntType::i128_type();
    let f16_type = FloatType::f16_type();
    let f32_type = FloatType::f32_type();
    let f64_type = FloatType::f64_type();
    let f128_type = FloatType::f128_type();
    let ppc_f128_type = FloatType::ppc_f128_type();
    let struct_type = StructType::struct_type(&[&i8_type, &f128_type], false);
    let struct_type2 = StructType::struct_type(&[], false);
    let struct_type3 = StructType::struct_type(&[&i8_type, &f128_type], true);
    let struct_type4 = StructType::struct_type(&[], true);
    let opaque_struct_type = context.opaque_struct_type("opaque");
    let fn_type = void_type.fn_type(&[], false);
    let fn_type2 = i8_type.fn_type(&[], false);
    let fn_type3 = void_type.fn_type(&[&i32_type, &struct_type], false);
    let fn_type4 = i8_type.fn_type(&[&struct_type, &i32_type], false);

    // REVIEW: Should these maybe just be constant functions instead of bothering to calling LLVM?

    assert!(!void_type.is_sized());
    assert!(bool_type.is_sized());
    assert!(i8_type.is_sized());
    assert!(i16_type.is_sized());
    assert!(i32_type.is_sized());
    assert!(i64_type.is_sized());
    assert!(i128_type.is_sized());
    assert!(f16_type.is_sized());
    assert!(f32_type.is_sized());
    assert!(f64_type.is_sized());
    assert!(f128_type.is_sized());
    assert!(ppc_f128_type.is_sized());
    assert!(struct_type.is_sized());
    assert!(struct_type2.is_sized());
    assert!(struct_type3.is_sized());
    assert!(struct_type4.is_sized());
    assert!(!opaque_struct_type.is_sized());
    assert!(!fn_type.is_sized());
    assert!(!fn_type2.is_sized());
    assert!(!fn_type3.is_sized());
    assert!(!fn_type4.is_sized());

    assert!(void_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(bool_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(i8_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(i16_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(i32_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(i64_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(i128_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(f16_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(f32_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(f64_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(f128_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(ppc_f128_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(struct_type.ptr_type(AddressSpace::Generic).is_sized());
    assert!(struct_type2.ptr_type(AddressSpace::Generic).is_sized());
    assert!(struct_type3.ptr_type(AddressSpace::Generic).is_sized());
    assert!(struct_type4.ptr_type(AddressSpace::Generic).is_sized());
    assert!(opaque_struct_type.ptr_type(AddressSpace::Generic).is_sized());

    // REVIEW: You can't have array of void right?
    assert!(void_type.ptr_type(AddressSpace::Generic).array_type(42).is_sized());
    assert!(bool_type.array_type(42).is_sized());
    assert!(i8_type.array_type(42).is_sized());
    assert!(i16_type.array_type(42).is_sized());
    assert!(i32_type.array_type(42).is_sized());
    assert!(i64_type.array_type(42).is_sized());
    assert!(i128_type.array_type(42).is_sized());
    assert!(f16_type.array_type(42).is_sized());
    assert!(f32_type.array_type(42).is_sized());
    assert!(f64_type.array_type(42).is_sized());
    assert!(f128_type.array_type(42).is_sized());
    assert!(ppc_f128_type.array_type(42).is_sized());
    assert!(struct_type.array_type(0).is_sized());
    assert!(struct_type2.array_type(0).is_sized());
    assert!(struct_type3.array_type(0).is_sized());
    assert!(struct_type4.array_type(0).is_sized());
    assert!(!opaque_struct_type.array_type(0).is_sized());

    // REVIEW: You can't have vec of void right?
    assert!(void_type.ptr_type(AddressSpace::Generic).vec_type(42).is_sized());
    assert!(bool_type.vec_type(42).is_sized());
    assert!(i8_type.vec_type(42).is_sized());
    assert!(i16_type.vec_type(42).is_sized());
    assert!(i32_type.vec_type(42).is_sized());
    assert!(i64_type.vec_type(42).is_sized());
    assert!(i128_type.vec_type(42).is_sized());
    assert!(f16_type.vec_type(42).is_sized());
    assert!(f32_type.vec_type(42).is_sized());
    assert!(f64_type.vec_type(42).is_sized());
    assert!(f128_type.vec_type(42).is_sized());
    assert!(ppc_f128_type.vec_type(42).is_sized());
    assert!(struct_type.vec_type(42).is_sized());
    assert!(struct_type2.vec_type(42).is_sized());
    assert!(struct_type3.vec_type(42).is_sized());
    assert!(struct_type4.vec_type(42).is_sized());
    assert!(!opaque_struct_type.vec_type(42).is_sized());
}

#[test]
fn test_const_null() {
    let context = Context::create();
    let void_type = context.void_type();
    let bool_type = context.bool_type();
    let i8_type = context.i8_type();
    let i16_type = context.i16_type();
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let i128_type = context.i128_type();
    let f16_type = context.f16_type();
    let f32_type = context.f32_type();
    let f64_type = context.f64_type();
    let f128_type = context.f128_type();
    let struct_type = context.struct_type(&[&i8_type, &f128_type], false);
    let ptr_type = f64_type.ptr_type(AddressSpace::Generic);
    let vec_type = f64_type.vec_type(42);
    let array_type = f64_type.array_type(42);

    assert!(void_type.const_null_ptr().is_null());
    assert!(bool_type.const_null_ptr().is_null());
    assert!(i8_type.const_null_ptr().is_null());
    assert!(i16_type.const_null_ptr().is_null());
    assert!(i32_type.const_null_ptr().is_null());
    assert!(i64_type.const_null_ptr().is_null());
    assert!(i128_type.const_null_ptr().is_null());
    assert!(f16_type.const_null_ptr().is_null());
    assert!(f32_type.const_null_ptr().is_null());
    assert!(f64_type.const_null_ptr().is_null());
    assert!(f128_type.const_null_ptr().is_null());
    assert!(struct_type.const_null_ptr().is_null());
    assert!(ptr_type.const_null_ptr().is_null());
    assert!(vec_type.const_null_ptr().is_null());
    assert!(array_type.const_null_ptr().is_null());

    let bool_zero = bool_type.const_null();
    let i8_zero = i8_type.const_null();
    let i16_zero = i16_type.const_null();
    let i32_zero = i32_type.const_null();
    let i64_zero = i64_type.const_null();
    let i128_zero = i128_type.const_null();
    let f16_zero = f16_type.const_null();
    let f32_zero = f32_type.const_null();
    let f64_zero = f64_type.const_null();
    let f128_zero = f128_type.const_null();
    let struct_zero = struct_type.const_null();
    let ptr_zero = ptr_type.const_null();
    let vec_zero = vec_type.const_null();
    let array_zero = array_type.const_null();

    assert_eq!(bool_zero.print_to_string(), &*CString::new("i1 false").unwrap());
    assert!(bool_zero.is_null());
    assert_eq!(i8_zero.print_to_string(), &*CString::new("i8 0").unwrap());
    assert!(i8_zero.is_null());
    assert_eq!(i16_zero.print_to_string(), &*CString::new("i16 0").unwrap());
    assert!(i16_zero.is_null());
    assert_eq!(i32_zero.print_to_string(), &*CString::new("i32 0").unwrap());
    assert!(i32_zero.is_null());
    assert_eq!(i64_zero.print_to_string(), &*CString::new("i64 0").unwrap());
    assert!(i64_zero.is_null());
    assert_eq!(i128_zero.print_to_string(), &*CString::new("i128 0").unwrap());
    assert!(i128_zero.is_null());
    assert_eq!(f16_zero.print_to_string(), &*CString::new("half 0xH0000").unwrap());
    assert!(f16_zero.is_null());
    assert_eq!(f32_zero.print_to_string(), &*CString::new("float 0.000000e+00").unwrap());
    assert!(f32_zero.is_null());
    assert_eq!(f64_zero.print_to_string(), &*CString::new("double 0.000000e+00").unwrap());
    assert!(f64_zero.is_null());
    assert_eq!(f128_zero.print_to_string(), &*CString::new("fp128 0xL00000000000000000000000000000000").unwrap());
    assert!(f128_zero.is_null());
    assert_eq!(struct_zero.print_to_string(), &*CString::new("{ i8, fp128 } zeroinitializer").unwrap());
    assert!(struct_zero.is_null());
    assert_eq!(ptr_zero.print_to_string(), &*CString::new("double* null").unwrap());
    assert!(ptr_zero.is_null());
    assert_eq!(vec_zero.print_to_string(), &*CString::new("<42 x double> zeroinitializer").unwrap());
    assert!(vec_zero.is_null());
    assert_eq!(array_zero.print_to_string(), &*CString::new("[42 x double] zeroinitializer").unwrap());
    assert!(array_zero.is_null());
}

#[test]
fn test_vec_type() {
    let context = Context::create();
    let int = context.i8_type();
    let vec_type = int.vec_type(42);

    assert_eq!(vec_type.get_size(), 42);
}

#[test]
fn test_type_copies() {
    let context = Context::create();
    let i8_type = context.i8_type();
    let i8_type_copy = i8_type;

    assert_eq!(i8_type, i8_type_copy);
}
