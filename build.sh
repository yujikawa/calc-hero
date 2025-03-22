#!/bin/bash
set -e

cargo build --target wasm32-unknown-unknown

# 出力された .wasm を static にコピー
cp target/wasm32-unknown-unknown/debug/calc-hero.wasm static/
