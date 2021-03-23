@echo off
cargo run
:cls
ctf_vm.exe build new.cbm new1.cbm -o
echo Running unoptimized
ctf_vm.exe run new.cbm