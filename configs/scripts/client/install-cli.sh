#!/bin/bash

WORKING_DIR=$(pwd)

# Installs to local Rust bin directory
cargo install --path ${WORKING_DIR}/clients/cli
