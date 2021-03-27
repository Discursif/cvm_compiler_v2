@echo off
cargo run
:cls
echo Running unoptimized
"cvm/vm/ctf_vm.exe" run cvm/build/new.cbm