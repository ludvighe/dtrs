#!/bin/bash -eu

target_path="target/release/dt"
install_path="$HOME/.local/bin/"
cargo build --release && sudo cp $target_path $install_path && echo -e "\n✓ dt installed to $install_path"
