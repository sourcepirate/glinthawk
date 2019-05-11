#!/bin/bash

echo "Builing the debian package."
mkdir -p usr/bin
cp ../target/$1/glinthawk_cli glinthawk/usr/bin/
dpkg-deb --build glinthawk
cp glinthawk.deb /release/