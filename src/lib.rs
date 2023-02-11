pub mod cli;

use crate::cli::Cli;
use bio::alignment::pairwise::Aligner;
use bio::alignment::{Alignment, AlignmentMode, AlignmentOperation};
use bio::io::fasta::{Reader, Record};
use bio::scores::blosum62;
use crossbeam::channel;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Some types to simplify things.
type RecordPairSender = channel::Sender<(Record, Record)>;
struct Workers {
    thread_handles: Vec<JoinHandle<()>>,
    senders: Vec<RecordPairSender>,
}

/// Double check the alignment assumptions!
fn assert_global(x: &Record, y: &Record, alignment: &Alignment) {
    assert_eq!(alignment.xstart, 0);
    assert_eq!(alignment.xend, x.seq().len());
    assert_eq!(alignment.ystart, 0);
    assert_eq!(alignment.yend, y.seq().len());

    match alignment.mode {
        AlignmentMode::Global => (),
        _ => panic!("should be global"),
    };
}

/// Count identities/matches in the alignment.
fn count_identities(alignment: &Alignment) -> i32 {
    let n = alignment
        .operations
        .iter()
        .fold(0u64, |count, op| match op {
            AlignmentOperation::Match => count + 1,
            _ => count,
        });

    // This should never fail as inteins are short.
    i32::try_from(n).unwrap()
}

/// The length of the alignment is the number of alignment operations.
fn alignment_length(alignment: &Alignment) -> i32 {
    let len = alignment.operations.len();

    // This should never fail as inteins are short.
    i32::try_from(len).unwrap()
}

/// Percent identity is the number of matches divided by the alignment length.
fn percent_identity(aln_len: i32, num_matches: i32) -> f64 {
    f64::from(num_matches) / f64::from(aln_len)
}

/// Print one line with info for alignment.
fn print_alignment_line(
    out: &mut BufWriter<File>,
    x: &Record,
    y: &Record,
    aln_len: i32,
    num_matches: i32,
    percent_identity: f64,
) {
    let x_name = x.id();
    let y_name = y.id();

    let x_len = x.seq().len();
    let y_len = y.seq().len();

    writeln!(
        out,
        "{x_name}\t{y_name}\t{x_len}\t{y_len}\t{aln_len}\t{num_matches}\t{percent_identity}"
    )
    .unwrap();
}

/// Print the tab-separated results of the alignment.
fn print_alignment_info(
    out: &Mutex<BufWriter<File>>,
    x: &Record,
    y: &Record,
    alignment: &Alignment,
) {
    let aln_len = alignment_length(alignment);
    let num_matches = count_identities(alignment);

    let percent_identity = percent_identity(aln_len, num_matches);

    let stdout = &mut *(out.lock().unwrap());
    print_alignment_line(stdout, x, y, num_matches, aln_len, percent_identity);
    print_alignment_line(stdout, y, x, num_matches, aln_len, percent_identity);
}

fn get_records(path: PathBuf) -> Vec<Record> {
    let file = File::open(path).unwrap();
    let reader = Reader::new(file);

    reader
        .records()
        .map(|x| {
            let x = x.unwrap();
            // Some of the inteins have lowercase AA residues.  This breaks the alignment.
            let uppercase_seq = x.seq().to_ascii_uppercase();
            Record::with_attrs(x.id(), x.desc(), uppercase_seq.as_slice())
        })
        .collect::<Vec<Record>>()
}

/// Set up the worker threads and channels.
fn set_up_workers(
    num_threads: usize,
    gap_open: i32,
    gap_extend: i32,
    out: Arc<Mutex<BufWriter<File>>>,
) -> Workers {
    let mut thread_handles = Vec::with_capacity(num_threads);
    let mut senders = Vec::with_capacity(num_threads);

    (0..num_threads).for_each(|_| {
        let (s, r) = channel::bounded::<(Record, Record)>(256);

        let out = out.clone();
        let handle = thread::spawn(move || {
            let mut aligner = Aligner::new(gap_open, gap_extend, &blosum62);
            for (x, y) in r {
                let alignment = aligner.global(x.seq(), y.seq());
                assert_global(&x, &y, &alignment);
                print_alignment_info(&out, &x, &y, &alignment);
            }
        });

        senders.push(s);
        thread_handles.push(handle);
    });

    Workers {
        thread_handles,
        senders,
    }
}

/// Self-hits don't need alignment, so write out the equal sequence alignment info.
fn write_self_hits(records: &[Record], out: Arc<Mutex<BufWriter<File>>>) {
    let mut out = out.lock().unwrap();
    records.iter().for_each(|r| {
        // Safe because inteins are short.
        let len = i32::try_from(r.seq().len()).unwrap();
        print_alignment_line(&mut out, r, r, len, len, 1.0);
    });
}

/// Align records
///
/// The worker threads will handle file output.
///
/// The senders will drop at the end of this function, so you don't have to manually close them.
fn align_records(records: Vec<Record>, senders: Vec<RecordPairSender>, num_threads: usize) {
    records
        .iter()
        .tuple_combinations::<(_, _)>()
        .enumerate()
        .for_each(|(i, (x, y))| {
            let tx = &senders[i % num_threads];

            tx.send((x.clone(), y.clone())).unwrap();
        });
}

pub fn run(cli: Cli) {
    let out = File::create(cli.out_file.clone()).unwrap();
    let out = Arc::new(Mutex::new(BufWriter::new(out)));

    let records = get_records(cli.in_file.clone());

    let num_threads = cli.threads();
    let Workers {
        thread_handles,
        senders,
    } = set_up_workers(num_threads, cli.gap_open(), cli.gap_extend(), out.clone());

    write_self_hits(&records, out);
    align_records(records, senders, num_threads);

    // Wait for the threads to finish working.
    thread_handles.into_iter().for_each(|t| t.join().unwrap());
}
