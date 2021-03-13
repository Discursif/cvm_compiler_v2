@echo off
cargo run
pause >nul
cls
ctf_vm.exe build new.cbm new1.cbm -o -p 150
pause >nul
cls
echo Running unoptimized
ctf_vm.exe run new.cbm
pause >nul
cls
echo Running optimized
ctf_vm.exe run new1.cbm