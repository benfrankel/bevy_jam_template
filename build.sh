#!/bin/bash

set -u
shopt -s globstar

NAME='bevy_jam_template'
BUILD_DIR='build'

function usage() {
  echo "Usage: $0 [web|windows|linux|mac]..."
}

# Web
function web() {
  PLATFORM='web'
  TARGET='wasm32-unknown-unknown'
  OUT_DIR="${BUILD_DIR}/${PLATFORM}"
  OUT_ZIP="${BUILD_DIR}/${NAME}_${PLATFORM}.zip"

  # Clear output location
  mkdir -p "${OUT_DIR}"
  rm -rf "${OUT_DIR:?}/"* "${OUT_ZIP}"

  # Build
  cargo build --profile=wasm-release --features=web --target="${TARGET}"
  wasm-bindgen --no-typescript --out-name run --out-dir "${OUT_DIR}" --target web "target/${TARGET}/wasm-release/run.wasm"
  wasm-opt -O -ol 100 -s 100 -o "${OUT_DIR}/run_bg.wasm" "${OUT_DIR}/run_bg.wasm"

  # Prepare zip
  cp -r assets web/* "${OUT_DIR}"
  rm "${OUT_DIR:?}"/**/*.aseprite
  zip -r "${OUT_ZIP}" "${OUT_DIR}"
}

# Windows
function windows() {
  PLATFORM='windows'
  TARGET='x86_64-pc-windows-gnu'
  OUT_DIR="${BUILD_DIR}/${PLATFORM}"
  OUT_ZIP="${BUILD_DIR}/${NAME}_${PLATFORM}.zip"

  # Clear output location
  mkdir -p "${OUT_DIR}"
  rm -rf "${OUT_DIR:?}"/* "${OUT_ZIP}"

  # Build
  cargo build --release --features=native --target="${TARGET}"

  # Prepare zip
  cp -r assets "target/${TARGET}/release/run.exe" "${OUT_DIR}"
  rm "${OUT_DIR:?}"/**/*.aseprite
  zip -r "${OUT_ZIP}" "${OUT_DIR}"
}

# Linux
function linux() {
  PLATFORM='linux'
  TARGET='x86_64-unknown-linux-gnu'
  OUT_DIR="${BUILD_DIR}/${PLATFORM}"
  OUT_ZIP="${BUILD_DIR}/${NAME}_${PLATFORM}.zip"

  # Clear output location
  mkdir -p "${OUT_DIR}"
  rm -rf "${OUT_DIR:?}"/* "${OUT_ZIP}"

  # Build
  cargo build --release --features=native,bevy/wayland --target="${TARGET}"

  # Prepare zip
  cp -r assets "target/${TARGET}/release/run" "${OUT_DIR}"
  rm "${OUT_DIR:?}"/**/*.aseprite
  zip -r "${OUT_ZIP}" "${OUT_DIR}"
}

# FIXME: Requires a cross-compiler?
# Mac
function mac() {
  PLATFORM='mac'
  TARGET='x86_64-apple-darwin'
  OUT_DIR="${BUILD_DIR}/${PLATFORM}"
  OUT_ZIP="${BUILD_DIR}/${NAME}_${PLATFORM}.zip"

  # Clear output location
  mkdir -p "${OUT_DIR}"
  rm -rf "${OUT_DIR:?}"/* "${OUT_ZIP}"

  # Build
  cargo build --release --features=native --target="${TARGET}"

  # Prepare zip
  cp -r assets "target/${TARGET}/release/run.exe" "${OUT_DIR}"
  rm "${OUT_DIR:?}"/**/*.aseprite
  zip -r "${OUT_ZIP}" "${OUT_DIR}"
}

function main() {
  [ "$#" -eq 0 ] && usage

  while [ "$#" -ge 1 ]; do
    case "$1" in
    web) web ;;
    windows) windows ;;
    linux) linux ;;
    mac) mac ;;
    *) usage ;;
    esac

    shift 1
  done
}

main "$@"

exit 0
