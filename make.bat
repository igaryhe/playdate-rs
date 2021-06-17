@echo off
set out=%~dp0target\x86_64-pc-windows-msvc\release\examples
set out_device=%~dp0target\thumbv7em-none-eabihf\release\examples
if %1==simulator (
    mkdir %out%\source 2> nul
    copy /y nul %out%\source\pdex.bin > nul
    copy %out%\%2.dll %out%\source\pdex.dll > nul
    mkdir %~dp0build 2> nul
    start pdc %out%\source %~dp0build\%2.pdx
) else if %1==device (
    mkdir %out_device%\source 2> nul
    start arm-none-eabi-objcopy -O binary %out_device%\%2 %out_device%\source\pdex.bin
    mkdir %~dp0build 2> nul
    start pdc %out_device%\source %~dp0build\%2-device.pdx
)
