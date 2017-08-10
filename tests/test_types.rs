extern crate inkwell;

use std::ffi::CString;
use std::mem::transmute;

use self::inkwell::context::Context;
use self::inkwell::targets::{InitializationConfig, Target};
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
    assert!(!fn_type.is_sized());
    assert!(!fn_type2.is_sized());
    assert!(!fn_type3.is_sized());
    assert!(!fn_type4.is_sized());

    assert!(void_type.ptr_type(0).is_sized());
    assert!(bool_type.ptr_type(0).is_sized());
    assert!(i8_type.ptr_type(0).is_sized());
    assert!(i16_type.ptr_type(0).is_sized());
    assert!(i32_type.ptr_type(0).is_sized());
    assert!(i64_type.ptr_type(0).is_sized());
    assert!(i128_type.ptr_type(0).is_sized());
    assert!(f16_type.ptr_type(0).is_sized());
    assert!(f32_type.ptr_type(0).is_sized());
    assert!(f64_type.ptr_type(0).is_sized());
    assert!(f128_type.ptr_type(0).is_sized());
    assert!(ppc_f128_type.ptr_type(0).is_sized());
    assert!(struct_type.ptr_type(0).is_sized());
    assert!(struct_type2.ptr_type(0).is_sized());
    assert!(struct_type3.ptr_type(0).is_sized());
    assert!(struct_type4.ptr_type(0).is_sized());

    // REVIEW: You can't have array of void right?
    assert!(void_type.ptr_type(0).array_type(42).is_sized());
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

    // REVIEW: You can't have array of void right?
    assert!(void_type.ptr_type(0).vec_type(42).is_sized());
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

    let context = Context::get_global_context();
    let builder = context.create_builder();
    let module = context.create_module("sized");

    Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

    let execution_engine = module.create_execution_engine(true).unwrap();

    macro_rules! assert_size_eq {
        ($input:tt: $ty:ident($ty2:ident) == $size:expr) => {
            let fn_type = i64_type.fn_type(&[&$ty], false);
            let fn_value = module.add_function(concat!("get_", stringify!($ty)), &fn_type, None);

            let entry = fn_value.append_basic_block("entry");

            builder.position_at_end(&entry);

            let param = fn_value.get_first_param().unwrap().into_int_value();

            param.set_name("to_size");

            builder.build_return(Some(&param.get_type().size()));

            module.print_to_stderr();

            println!("{:?}", concat!("get_", stringify!($ty)));

            let addr = execution_engine.get_function_address(concat!("get_", stringify!($ty))).unwrap();

            let get_size: extern "C" fn($ty2) -> i64 = unsafe { transmute(addr) };

            assert_eq!(get_size($input), $size);
        }
    }

    assert_size_eq!(true: bool_type(bool) == 1);
    assert_size_eq!(false: bool_type(bool) == 1);
    assert_size_eq!(3: i8_type(i8) == 8);
    assert_size_eq!(3: i8_type(u8) == 8);




    // let bool_get_size_type = i64_type.fn_type(&[&bool_type], false);
    // let bool_get_size = module.add_function("bool_get_size", &bool_get_size_type, None);

    // let entry = bool_get_size.append_basic_block("entry");

    // builder.position_at_end(&entry);

    // let param = bool_get_size.get_first_param().unwrap().into_int_value();

    // param.set_name("to_size");

    // builder.build_return(Some(&param.get_type().size()));


    // let addr = execution_engine.get_function_address("bool_get_size").unwrap();

    // let get_size: extern "C" fn(bool) -> i64 = unsafe { transmute(addr) };

    // assert_eq!(get_size(true), 1);;
    // assert_eq!(get_size(false), 1);;

    // println!("{:?}", bool_type);
    // println!("{:?}", bool_type.size());
    // println!("{:?}", i8_type.size());
    // println!("{:?}", i16_type.size());
    // println!("{:?}", i32_type.size());
    // println!("{:?}", i64_type.size());
    // println!("{:?}", i128_type.size());
}

#[test]
fn test_vec_type() {
    let context = Context::create();
    let int = context.i8_type();
    let vec_type = int.vec_type(42);

    assert_eq!(vec_type.size(), 42);
}
