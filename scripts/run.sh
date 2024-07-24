#!/usr/bin/env bash

cd todo-rust || printf ""

cargo build

code=$?

if [ "$code" -ne 0 ]; then
	echo "could not compile exit: $code."
	exit 1
fi

echo "---Compiled!---"
echo ""

./target/debug/todo-rust

echo ""
echo ""
