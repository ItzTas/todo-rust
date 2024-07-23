#!/usr/bin/env bash

cd todo-rust || printf ""

cargo build

./target/debug/todo-rust
