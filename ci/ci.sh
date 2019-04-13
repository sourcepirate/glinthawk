#!/bin/bash

echo "Preparing Environment"

sudo apt-get update && apt-get -y install pkg-config \
			       libssl-dev \
			       build-essential \
			       openssl

sudo mkdir /release

# For building a static binary.
export OPENSSL_STATIC=1

echo "Building binary"

cargo build --release

echo "Pushing to release folder"

cp target/release/glinthawk_cli /release

echo "Spitting out to github release"
echo "Crate Name: ${CRATE_NAME}"
echo "Tag Info: ${TRAVIS_TAG}"

# Tag information available

cd /release

echo "Building a tar ball"
tar -cvf ${CRATE_NAME}-${TRAVIS_TAG}.tar.gz *

