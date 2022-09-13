#!/bin/sh
exec cargo run --bin update-profile --features=codegen -- "$@"
