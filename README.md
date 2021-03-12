# cvm_compiler_v2
 The second iteration of the CVM compiler
## Introduction
CVM is a high level language targetting compiling to CVM Bytecode which itself can be compiled to ANY target such as:
 > C, x86, ARM64, Python, JS...
CVM is similar to Rust in terms of syntax with touches of Java.
## Hello world
```rust
fn main() {
    print("Hello world!"~10);
}
```
Note: *The `10` is the ascii code for \n*
Note: *The `~` is an operator that merges two variables into one 0 1 ~ 2 3 = 0 1 2 3*

Note: *This example is usable without STD*
## Variables
CVM is staticly typed, this means all variables have a know type at compile time.
The widest type (equivalent of `Object` in Java) is `Bytes`.
```rust
fn main() {
    Bytes my_var_of_any_type = "test";

    // This overrides the `my_var_of_any_type` type
    String my_var_of_any_type = my_var_of_any_type;

    print("The size of `my_var_of_any_type` is "~my_var_of_any_type.len().as_str()~10);

    print("Hello world!"~10);
}
```
Note: *This example require STD*

## STD Types
CVM STD export few types like `Bytes`, `Byte`, `Empty`, `String`, `Char`, `Boolean`
The `Panic` type when constructed will make the compiler panic (This disable a method).

Bytes is the parent of every Type and has few methods:
```rust 
fn len() -> Byte;
fn as_str() -> String;
fn add(other: Bytes) -> Bytes;
fn sub(other: Bytes) -> Bytes;
fn div(other: Bytes) -> Bytes;
fn mul(other: Bytes) -> Bytes;
fn xor(other: Bytes) -> Bytes;
fn merge(other: Bytes) -> Bytes;
fn index(index: Byte) -> Byte;
fn index_range(index: Byte,index1: Byte) -> Bytes;
fn replace(index: Byte, object: Bytes) -> Bytes;
fn insert(index: Byte, object: Bytes) -> Bytes;
```

String has few methods
```rust
fn char_at(index: Byte) -> Char;
fn to_lower_case() -> String;
fn to_upper_case() -> String;
fn trim() -> String;
fn substring(start: Byte, end: Byte) -> String;
fn merge(other: String) -> String;
fn index(index: Byte) -> Char;
fn index_range(index: Byte,index1: Byte) -> String;
fn replace(index: Byte, object: String) -> String;
fn insert(index: Byte, object: String) -> Bytes;
```

Char has few methods
```rust
fn to_lower_case() -> String;
fn to_upper_case() -> String;
fn is_ascii() -> Bool;
fn is_number() -> Bool;
fn parse_hex() -> Byte;
fn merge(other: Char) -> String;
fn index(index: Byte) -> Panic;
fn index_range(index: Byte,index1: Byte) -> Panic;
fn replace(index: Byte, object: String) -> Panic;
fn insert(index: Byte, object: String) -> Panic;
```

