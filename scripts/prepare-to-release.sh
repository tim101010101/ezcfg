#!/bin/bash

set -e

cargo test
clear

cargo build --release
clear

TARGET_VERSION=""
TARGET_SHA=""

get_version() {
    TARGET_VERSION=$(grep '^version' crates/ezcfg/Cargo.toml | awk -F\" '{print $2}')
}

build_output() {
    echo "============= Building release output ================="

    # Create release output
    tar -czf target/release/ezcfg_${TARGET_VERSION}.tar.gz -C target/release ezcfg

    # Create sha256 checksum
    TARGET_SHA=$(shasum -a 256 target/release/ezcfg_${TARGET_VERSION}.tar.gz | awk '{print $1}')
    echo $TARGET_SHA > target/release/ezcfg_${TARGET_VERSION}-sha256.txt

    echo "version: $TARGET_VERSION"
    echo "sha256: $TARGET_SHA"
    echo ""
}

update_brew_pkg() {
    echo "================ Update brew package =================="

    sed -i '' "s/version \".*\"/version \"$TARGET_VERSION\"/" pkg/brew/ezcfg.rb
    sed -i '' "s/sha256 \".*\"/sha256 \"$TARGET_SHA\"/"       pkg/brew/ezcfg.rb

    echo "Update version $TARGET_VERSION"
    echo "Update sha $TARGET_SHA"
    echo ""
}

push_git_tag() {
    echo "=================== Update git tag ===================="

    git add .
    git commit -m "release: version $TARGET_VERSION"
    git tag -a $TARGET_VERSION -m "Release version $TARGET_VERSION"

    echo "Run 'git push origin --tags' to push the tag"
    echo ""
}

get_version

build_output

update_brew_pkg

push_git_tag