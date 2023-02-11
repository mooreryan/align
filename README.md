# Align

Perform all-vs-all global alignments for input protein sequences.

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