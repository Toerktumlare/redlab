#!/bin/sh
cargo build --release --target x86_64-pc-windows-gnu -j 24 &&
cp target/x86_64-pc-windows-gnu/release/redlab.exe . &&
exec ./redlab.exe "$@"
