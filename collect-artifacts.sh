#!/usr/bin/env bash

mkdir -p artifacts
cp target/release/wincron artifacts/wincron-release
cp target/debug/wincron artifacts/wincron-debug
