# Align

Calculate all vs. all percent identity for protein sequences using global alignments.

Use the `--threads` option to speed it up.

For info on the alignment, see [Rust-bio docs](https://docs.rs/bio/latest/bio/alignment/pairwise/struct.Aligner.html#method.global).

## Install

Requires [Rust](https://www.rust-lang.org/tools/install).

```
git clone https://github.com/mooreryan/align
cargo build --release
ln -s $(pwd)/target/release/align "$HOME/bin/align"
```

## Test

Requires [Just](https://just.systems/).

After cloning the repository, run

```
just test
```

## Example

Do all-vs-all alignments for the sequences in `seqs.faa`.

``` 
align --threads=4 seqs.faa out.tsv
```

Show help screen.

``` 
align --help
```

## Hacking

When you make a change to the code, such that the output of the test has changed, after verification, you should promote the new output to the expected output:

```
just promote
```

## License

[![license MIT or Apache
2.0](https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue)](https://github.com/mooreryan/InteinFinder)

Copyright (c) 2023 Ryan M. Moore

Licensed under the Apache License, Version 2.0 or the MIT license, at your option. This program may not be copied, modified, or distributed except according to those terms.
