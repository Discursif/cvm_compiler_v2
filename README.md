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
## Variables and conditions
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