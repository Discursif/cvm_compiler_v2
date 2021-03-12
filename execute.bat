@echo off
cargo run
pause >nul
ctf_vm.exe build new.cbm new1.cbm -o -p 150
pause >nul
ctf_vm.exe run new.cbm