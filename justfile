all: clippy check build release

help: build
  ./target/debug/align --help

clippy:
  cargo clippy

check:
  cargo check

test: build
  #!/usr/bin/env bash
  set -euxo pipefail

  OUT=test_files/rnr.aln.tsv
  EXPECTED=test_files/rnr.aln.tsv.expected

  if [ -e "${OUT}" ]; then
    rm "${OUT}"
  fi && \

  ./target/debug/align -t 4 test_files/rnr.fasta "${OUT}" && \

  diff <(sort "${OUT}") <(sort "${EXPECTED}") && \

  rm "${OUT}"

build:
  cargo build

release:
  cargo build --release