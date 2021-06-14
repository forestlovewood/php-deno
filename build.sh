#! /usr/bin/env bash

readonly RUST_VERSION="1.52.1"

function log {
  local COLOUR=""
  case $1 in
    error) COLOUR="\033[31m" ;;
    warning) COLOUR="\033[33m" ;;
    notice) COLOUR="\033[34m" ;;
    success) COLOUR="\033[32m" ;;
  esac

  local LOG="[libdeno] \033[1m${COLOUR}${2}\033[0m"

  if [ "$1" = "error" ]; then
    echo -e "$LOG" >&2
  else
    echo -e "$LOG"
  fi
}

function check_compatibility {
  log notice "Checking build environment compatibility..."

  RUST_ARCH=""
  case $(uname -m) in
    aarch64) RUST_ARCH="aarch64" ;;
    arm64) RUST_ARCH="aarch64" ;;
    x86_64) RUST_ARCH="x86_64" ;;
    amd64) RUST_ARCH="x86_64" ;;
    *)
      log error "Incompatible architecture. Build environment only supports Arm64 and x86_64."
      exit 1
    ;;
  esac

  RUST_FAMILY=""
  RUST_OS=""
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    RUST_FAMILY="unknown"
    if [[ -z $(ldd /bin/ls | grep 'musl' | head -1 | cut -d ' ' -f1) ]]; then
      RUST_OS="linux-gnu"
    else
      RUST_OS="linux-musl"
    fi
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    RUST_FAMILY="apple"
    RUST_OS="darwin"
  else
    log error "Incompatible operating system. Build environment only supports Linux and macOS."
    exit 1
  fi

  log success "Build environment compatible."
}

function create_environment {
  RUST_STDLIB="${RUST_ARCH}-${RUST_FAMILY}-${RUST_OS}"
  RUST_DIR="rust-${RUST_VERSION}-${RUST_STDLIB}"

  RUST_URI="https://static.rust-lang.org/dist/${RUST_DIR}.tar.gz"
  RUST_URI_ASC="${RUST_URI}.asc"

  RUST_TMP=$(mktemp)
  RUST_TMP_ASC=$(mktemp)
  RUST_TMP_BIN=$(mktemp -d)

  log notice "Downloading Rust ${RUST_VERSION} build environment..."
  (curl -sSf https://keybase.io/rust/pgp_keys.asc | gpg --quiet --import && curl -sSfo "${RUST_TMP}" $RUST_URI && curl -sSfo "${RUST_TMP_ASC}" $RUST_URI_ASC) || {
    log error "Could not download build environment."
    exit 1
  }
  log success "Build environment downloaded."

  log notice "Verifying build environment..."
  gpg --status-fd 1 --verify "${RUST_TMP_ASC}" "${RUST_TMP}" 2>/dev/null | grep -q "GOODSIG 85AB96E6FA1BE5FE" || {
    log error "Could not verify build environment."
    exit 1
  }
  log success "Build environment verified."

  log notice "Extracting build environment..."
  tar -xzf "${RUST_TMP}" -C "${RUST_TMP_BIN}" --strip-components=1
  rm -rf "${RUST_TMP}" "${RUST_TMP_ASC}"
  log success "Build environment extracted."

  export PATH="${RUST_TMP_BIN}/cargo/bin:${RUST_TMP_BIN}/rustc/bin:${PATH}"
  export RUSTFLAGS="-L ${RUST_TMP_BIN}/rust-std-${RUST_STDLIB}/lib/rustlib/${RUST_STDLIB}/lib"
}

function build_library {
  echo -e "[libdeno] ${F_BOLD}${F_YELLOW}Building library...${F_RESET}"
  cargo build --release --manifest-path=lib/Cargo.toml --quiet || {
    log error "Could not build library."
    exit 1
  }

  cp -f lib/target/release/libdeno.h lib

  if [[ "$RUST_OS" == "darwin" ]]; then
    cp -f lib/target/release/libdeno.dylib lib
  else
    cp -f lib/target/release/libdeno.so lib
  fi

  log success "Library build completed."
}

check_compatibility

log notice "Checking for existing build environment..."
if command -v rustc &> /dev/null && [[ $(php -r "echo version_compare('$(rustc --version | cut -d' ' -f 2)', '${RUST_VERSION}', '>=') ? 1 : 0;") = "1" ]]; then
  log success "Existing build environment found. Building with existing environment..."
else
  log warning "Existing build environment not found. Preparing temporary environment..."

  create_environment
fi

build_library
