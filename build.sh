#!/bin/bash
if [[ "$1" == "clean" ]] || if [[ "$1" == "simulator" ]] || if [[ "$1" == "device" ]];then
    if [[ "$1" == "clean" ]];then
        cargo clean
        rm -r build
    else
        if [[ "$1" == "simulator" ]];then
            target=x86_64-apple-darwin
        elif [[ "$1" == "device" ]];then
            target=thumbv7em-none-eabihf
        fi
        cargo build --release --example $2 --target $target
        out=target/$target/release/examples
        mkdir -p $out/source
        mkdir -p build
        if [[ "$1" == "simulator" ]];then
            touch $out/source/pdex.bin
            cp $out/lib$2.dylib $out/source/pdex.dylib
            pdc $out/source build/$2.pdx
        elif [[ "$1" == "device" ]];then
            arm-none-eabi-objcopy -O binary $out/$2 $out/source/pdex.bin
            pdc $out/source build/$2-device.pdx
        fi
    fi
fi