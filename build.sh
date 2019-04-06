#!/bin/bash

echo -e "Preparing environments"

sudo apt-get update

sudo apt-get install -y curl build-essential

sudo apt-get install pkg-config openssl libssl-dev

echo "Installing RUST environment...."
curl https://sh.rustup.rs -sSf | bash -s -- -y

source $HOME/.cargo/env
