# cvm_compiler_v2
 The second iteration of the CVM compiler
## Introduction
CVM is a high level language which compiles to CVM Bytecode which itself can be transpiled to all kind of targets such as:
 > C, x86, ARM64, Python, JS...
CVM is similar to Rust in terms of syntax with touches of Java.

## Aim of this project
This project was made to explore optimization and language theory.
The other aim of this project is making challenges for a custom CTF named Discursif.

## Why is CVM so particular to code with ?
Every feature of CVM is due or a consequence of the underlying possibilities or CVM Bytecode.
This is why it is very important to learn the basics of CVM Bytecode before programming in CVM.

The CVM Bytecode has only 9 instruction with the OP instruction having 9 kinds of suboperation.

In CVM registers have a variable size between 0 and 255. They act like an Vector of unsigned bytes with a maximum capacity of 255.

All operations in CVM are wrapping!
For instance: `250 + 10 = 5`
If you don't understand wrapping just imagine there is a modulus 256 after each operation.

Furthermore, registers when operating are cyclic this means:
```
10 20 30 40 50 60 70 + 1 2 = 11 22 31 42 51 62 71
```
As you can see if the second register is smaller than the first it will cycle to create a long enough register.
This means that operations in CVM aren't commutative.
```
10 + 20 1 = 30
20 1 + 10 = 30 11
```

Here all the possible instructions:
`CONST v<nreg>, <byte array>` This instruction sets the `<nreg>` register to the value of the `<byte array>`.
`MOV v<nreg1>, v<nreg2>` This instruction sets the `<nreg1>` register to the value of the `<nreg2>` register.
`IF v<nreg1>, v<nreg2>` If the register the `<nreg1>` is equal to the `<nreg2>` register we skip the next instuction.
`IFN v<nreg1>, v<nreg2>` If the register the `<nreg1>` is NOT equal to the `<nreg2>` register we skip the next instuction.
*Note that IF and IFN are encoded as one instruction in the binary*
`JUMP <line>` Jump to `<line>`
`END` Closes the program
`INPUT v<nreg>` Ask for user input (Will only take the first 255 bytes) and put it into `<nreg>` register
`PRINT v<nreg>` Print the `<nreg>` register as UTF8 sequence.
`LEN v<nreg1> v<nreg2>` Set the `<nreg1>` register to a register of size one which contains the `v<nreg2>` register size.
`READ v<nreg1> v<nreg2> v<nreg3> v<nreg4>` Set the `<nreg1>` register to the values of `v<nreg2>` register starting from first value of the `v<nreg3>` register and taking as many values as the first value of the `v<nreg4>` register.
`ADD v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping addition of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`SUB v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping substraction of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`MUL v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping multiplication of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`DIV v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping division of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`MOD v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping modulus of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`AND v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping binary and of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`OR v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping binary or of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`XOR v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to a wrapping binary xor of the `v<nreg2>` register with a cycle of the `v<nreg3>` register.
`MERGE v<nreg1> v<nreg2> v<nreg3>` Set the `<nreg1>` register to the concatenation of the `v<nreg2>` and the `v<nreg3>` registers.
*Note that all operations including MERGE are encoded in the same instruction*



## Hello world
```rust
fn main() {
    println("Hello world!");
}
```

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
