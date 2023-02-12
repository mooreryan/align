all: clippy check build release

help: build
  ./target/debug/align --help

clippy:
  cargo clippy

check:
  cargo check

test_rnr: build
  #!/usr/bin/env bash
  set -euxo pipefail

  IN=test_files/rnr.fasta
  OUT=test_files/rnr.aln.tsv
  EXPECTED=test_files/rnr.aln.tsv.expected

  if [ -e "${OUT}" ]; then
    rm "${OUT}"
  fi && \

  ./target/debug/align -t 4 "${IN}" "${OUT}" && \

  diff <(sort "${OUT}") <(sort "${EXPECTED}")

test_tiny: build
  #!/usr/bin/env bash
  set -euxo pipefail

  IN=test_files/tiny.fasta
  OUT=test_files/tiny.aln.tsv
  EXPECTED=test_files/tiny.aln_w_ops.tsv.expected

  if [ -e "${OUT}" ]; then
    rm "${OUT}"
  fi && \

  ./target/debug/align --show-aln-ops -t 4 "${IN}" "${OUT}" && \

  diff <(sort "${OUT}") <(sort "${EXPECTED}")

test: test_rnr test_tiny

promote_rnr:
  #!/usr/bin/env bash
  set -euxo pipefail

  OUT=test_files/rnr.aln.tsv
  EXPECTED=test_files/rnr.aln.tsv.expected

  if [ -e "${OUT}" ]; then
    mv "${OUT}" ${EXPECTED}
  fi

promote_tiny:
  #!/usr/bin/env bash
  set -euxo pipefail

  OUT=test_files/tiny.aln.tsv
  EXPECTED=test_files/tiny.aln_w_ops.tsv.expected

  if [ -e "${OUT}" ]; then
    mv "${OUT}" ${EXPECTED}
  fi

promote: promote_rnr promote_tiny

clean_test:
  rm test_files/*.tsv

build:
  cargo build

release:
  cargo build --release