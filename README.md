# EnvTime

This crate provides a procedural macro that can retrieve an environment variable at compile time or runtime.

Specify an environment variable at compile time and the runtime variable will never be evaluated.
Don't specify the environment variable at compile time, and it will be evaluated at runtime.

It is up to the compiler to then optimize and remove code if the compile time variable is set.

## Installation

```toml
[dependencies]
envtime = "0.0.1"
```

## Syntax & Usage
```rs
use envtime::*;

// You can use unwrap_or_else on the envtime macro
let var = envtime!("TEST_NON_ENV").unwrap_or_else(|| String::from("hello"));
assert_eq!(var, String::from("012"));
// This resolves to "hello" assuming it is not defined at compile time or runtime 

// Lets set a runtime variable to "123"
env::set_var("TEST_RUN_ENV", "123");
let var = envtime!("TEST_RUN_ENV");
assert_eq!(var, Some(String::from("123")));
// And you can see it gets the value

// Assume we have a compile time variable set to "456"
env::set_var("TEST_COMP_ENV", "123"); // We set the runtime variable to "123"
let var = envtime!("TEST_COMP_ENV");
assert_eq!(var, Some(String::from("456")));
// And the runtime variable is ignored, as the macro resolves to 'String::from("456")' at compile time

// Assume we don't have the runtime variable set at first, you can see the default value being used
assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "test");
env::set_var("TEST_STR_RUN_ENV", "not");
assert_eq!(envtime_def!("TEST_STR_RUN_ENV", "test"), "not");
// And once it is set it resolves to our runtime value

// Assume we have "TEST_BOOL_COMP_ENV" at compile time to "true"
env::set_var("TEST_BOOL_COMP_ENV", "false"); // Sets the runtime-variable, which is ignored
let enable_test = envtime_def!("TEST_BOOL_COMP_ENV", false);  // Resolves to the literal "true"
assert_eq!(enable_test, true);
// With the default value being false, and the runtime value being false, it still evaluates to true

// Example with u8
assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 77u8);
env::set_var("TEST_U8_RUN_ENV", "53");
assert_eq!(envtime_def!("TEST_U8_RUN_ENV", 77u8), 53u8);

```

## Note
For integer literals it is strongly suggested you include the suffixes "u8" / "i8" / "u16" / "i16" etc.
For string literals a String::from() is always used due to the difference in compile time and runtime environments.

## License

This project is licensed under the [MIT license].

[mit license]: https://github.com/AsyncVoid/envtime/blob/master/LICENSE