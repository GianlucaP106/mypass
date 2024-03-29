#! /bin/bash

mkdir ~/.mypass/ 2>/dev/null | true

cd ~/.mypass

rm -rf src

VERSION="0.1.0"

curl -fsSL https://github.com/GianlucaP106/mypass/archive/refs/tags/v${VERSION}.zip -o src.zip

unzip -d src "src.zip"
rm -rf "src.zip"

cd "src/mypass-${VERSION}"
cargo build -r
cp ./target/release/mypass ..
