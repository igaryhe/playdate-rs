@echo off
if not %1==clean if not %1==simulator if not %1==device goto:eof
if %1==clean (
    start /b cargo clean
    rmdir /s /q %~dp0build 2> nul
    goto:eof
)
if %1==simulator (
    set target=x86_64-pc-windows-msvc
) else if %1==device (
    set target=thumbv7em-none-eabihf
)
set out=%~dp0target\%target%\release\examples
start /w /b cargo build --release --example %2 --target %target%
if %errorlevel% neq 0 exit %errorlevel%
if not exist %out%\source mkdir %out%\source
if not exist %~dp0build mkdir %~dp0build
if %1==simulator (
    copy /y nul %out%\source\pdex.bin > nul
    copy %out%\%2.dll %out%\source\pdex.dll > nul
    start pdc %out%\source %~dp0build\%2.pdx
) else if %1==device (
    start /w /b arm-none-eabi-objcopy -O binary %out%\%2 %out%\source\pdex.bin
    start pdc %out%\source %~dp0build\%2-device.pdx
)
