use std::env;
use envtime::*;

#[test]
fn main_tests() {
    let var = envtime!("TEST_NON_ENV").unwrap_or_else(|| String::from("012"));
    assert_eq!(var, String::from("012"));

    env::set_var("TEST_RUN_ENV", "123");
    let var = envtime!("TEST_RUN_ENV");
    assert_eq!(var, Some(String::from("123")));

    env::set_var("TEST_COMP_ENV", "123");
    let var = envtime!("TEST_COMP_ENV");
    assert_eq!(var, Some(String::from("456")));
}

#[test]
fn compilation_def_tests() {
    env::set_var("TEST_BOOL_COMP_ENV", "false"); // Setting runtime-variable, ignored
    let var = envtime_def!("TEST_BOOL_COMP_ENV", false);
    assert_eq!(var, true);

    let var = envtime_def!("TEST_BYTE_COMP_ENV", b'a');
    assert_eq!(var, 10u8);

    env::set_var("TEST_U8_COMP_ENV", "0");
    let var = envtime_def!("TEST_U8_COMP_ENV", 50u8);
    assert_eq!(var, 12u8);

    env::set_var("TEST_I128_COMP_ENV", "10");
    let var = envtime_def!("TEST_I128_COMP_ENV", 50i128);
    assert_eq!(var, 25i128);
}

#[test]
fn runtime_def_tests() {
    assert_eq!(envtime_def!("TEST_BOOL_RUN_ENV", true), true);
    env::set_var("TEST_BOOL_RUN_ENV", "false");
    assert_eq!(envtime_def!("TEST_BOOL_RUN_ENV", true), false);

    assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "test");
    env::set_var("TEST_STR_RUN_ENV", "not");
    assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "not");

    assert_eq!(envtime_def!("TEST_BYTE_RUN_ENV", b'a'), b'a');
    env::set_var("TEST_BYTE_RUN_ENV", "53");
    assert_eq!(envtime_def!("TEST_BYTE_RUN_ENV", b'a'), 53u8);

    assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 77u8);
    env::set_var("TEST_U8_RUN_ENV", "53");
    assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 53u8);

    assert_eq!(envtime_def!("TEST_I8_RUN_ENV", -5i8), -5i8);
    env::set_var("TEST_I8_RUN_ENV", "-54");
    assert_eq!(envtime_def!("TEST_I8_RUN_ENV", -5i8), -54i8);

    assert_eq!(envtime_def!("TEST_U16_RUN_ENV", 312u16), 312u16);
    env::set_var("TEST_U16_RUN_ENV", "432");
    assert_eq!(envtime_def!("TEST_U16_RUN_ENV", 312u16), 432u16);

    assert_eq!(envtime_def!("TEST_I16_RUN_ENV", -674i16), -674i16);
    env::set_var("TEST_I16_RUN_ENV", "-768");
    assert_eq!(envtime_def!("TEST_I16_RUN_ENV", -674i16), -768i16);

    assert_eq!(envtime_def!("TEST_U32_RUN_ENV", 2343546u32), 2343546u32);
    env::set_var("TEST_U32_RUN_ENV", "78356765");
    assert_eq!(envtime_def!("TEST_U32_RUN_ENV", 2343546u32), 78356765u32);

    assert_eq!(envtime_def!("TEST_I32_RUN_ENV", -345568657i32), -345568657i32);
    env::set_var("TEST_I32_RUN_ENV", "-565782768");
    assert_eq!(envtime_def!("TEST_I32_RUN_ENV", -345568657i32), -565782768i32);

    assert_eq!(envtime_def!("TEST_U64_RUN_ENV", 47456877846789u64), 47456877846789u64);
    env::set_var("TEST_U64_RUN_ENV", "89457345673456");
    assert_eq!(envtime_def!("TEST_U64_RUN_ENV", 47456877846789u64), 89457345673456u64);

    assert_eq!(envtime_def!("TEST_I64_RUN_ENV", -5635675681006i64), -5635675681006i64);
    env::set_var("TEST_I64_RUN_ENV", "-1345667679780");
    assert_eq!(envtime_def!("TEST_I64_RUN_ENV", -5635675681006i64), -1345667679780i64);

    assert_eq!(envtime_def!("TEST_U128_RUN_ENV", 987234578934529873452978u128), 987234578934529873452978u128);
    env::set_var("TEST_U128_RUN_ENV", "783457823548976345296783");
    assert_eq!(envtime_def!("TEST_U128_RUN_ENV", 987234578934529873452978u128), 783457823548976345296783u128);

    assert_eq!(envtime_def!("TEST_I128_RUN_ENV", -90234513046340598234675i128), -90234513046340598234675i128);
    env::set_var("TEST_I128_RUN_ENV", "-12345983458945603456064");
    assert_eq!(envtime_def!("TEST_I128_RUN_ENV", -90234513046340598234675i128), -12345983458945603456064);
}