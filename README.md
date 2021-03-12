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
CVM STD export few types like `Bytes`, `Byte`, `Empty`, `String`, `Char`, `Bool`
The `Panic` type when constructed will make the compiler panic (This disable a method).

Bytes is the parent of every Type and has few methods:
```rust 
fn len() -> Byte;
fn as_str() -> String;
fn add(Bytes other) -> Bytes;
fn sub(Bytes other) -> Bytes;
fn div(Bytes other) -> Bytes;
fn mul(Bytes other) -> Bytes;
fn xor(Bytes other) -> Bytes;
fn merge(Bytes other) -> Bytes;
fn index(Byte index) -> Byte;
fn index_range(Byte index,Byte index1) -> Bytes;
fn replace(Byte index, object: Bytes) -> Bytes;
fn insert(Byte index, object: Bytes) -> Bytes;
```

String has few methods
```rust
fn char_at(Byte index) -> Char;
fn to_lower_case() -> String;
fn to_upper_case() -> String;
fn trim() -> String;
fn substring(Byte start, Byte end) -> String;
fn merge(other: String) -> String;
fn index(Byte index) -> Char;
fn index_range(Byte index,Byte index1) -> String;
fn replace(Byte index, String object) -> String;
fn insert(Byte index, String object) -> Bytes;
```

Char has few methods
```rust
fn to_lower_case() -> String;
fn to_upper_case() -> String;
fn is_ascii() -> Bool;
fn is_number() -> Bool;
fn parse_hex() -> Byte;
fn merge(Char other) -> String;
fn index(Byte index) -> Panic;
fn index_range(Byte index,Byte index1) -> Panic;
fn replace(Byte index, String object) -> Panic;
fn insert(Byte index, String object) -> Panic;
```

Boolean is defined as follows
```rust
type Boolean = 1 {
    ref true = 1;
    ref false = 0;

    fn merge(Bytes other) -> Panic;
    fn index(Byte index) -> Panic;
    fn index_range(Byte index,Byte index1) -> Panic;
    fn replace(Byte index, Bytes object) -> Panic;
    fn insert(Byte index, Bytes object) -> Panic;
}
```
