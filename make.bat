@echo off
set out=%~dp0target\release\examples
set out_device=%~dp0target\thumbv7em-none-eabihf\release\examples
if %1==simulator (
    mkdir %out%\source > nul
    copy /y nul %out%\source\pdex.bin
    copy %out%\%2.dll %out%\source\pdex.dll
    start pdc %out%\source %~dp0out.pdx
) else if %1==device (
    mkdir %out_device%\source > nul
    start arm-none-eabi-objcopy -O binary %out_device%\hello_world %out_device%\source\pdex.bin
    start pdc %out_device%\source %~dp0out.pdx
)