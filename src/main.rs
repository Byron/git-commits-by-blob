extern crate failure;
extern crate failure_tools;
extern crate git2;
extern crate indicatif;
#[macro_use]
extern crate structopt;
extern crate crossbeam;
extern crate num_cpus;

use failure::Error;
use failure_tools::ok_or_exit;
use std::{collections::BTreeMap, io::{stdin, stdout, BufRead, BufReader, Write}, path::PathBuf};
use git2::{ObjectType, Oid};
use structopt::StructOpt;

mod lut;

#[derive(Clone)]
pub enum Capsule {
    Normal(Vec<Oid>),
    Compact(Vec<usize>),
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "git-commits-by-blob")]
pub struct Options {
    /// The amount of threads to use. If unset, defaults to amount of physical CPUs
    #[structopt(short = "t", long = "threads")]
    threads: Option<usize>,

    /// If set, you will trade in about 35% increase in memory for about 30% less time till ready
    /// for queries
    #[structopt(long = "no-compact")]
    no_compact: bool,

    /// If set, traversal will only happen along the checked-out head.
    /// Otherwise it will take into consideration all remote branches, too
    /// Also useful for bare-repositories
    #[structopt(long = "head-only")]
    head_only: bool,

    /// the repository to index for queries
    #[structopt(name = "REPOSITORY", parse(from_os_str))]
    repository: PathBuf,

    /// The directory tree for which to figure out the merge commit.
    /// If unspecified, the program will serve as blob-to-commits lookup table,
    /// receiving hex-shas of blobs, one per line, on stdin and outputting
    /// all commits knowing that blob on stdout, separated by space, terminated
    /// by newline.
    #[structopt(name = "tree-to-integrate", parse(from_os_str))]
    tree: Option<PathBuf>,
}

fn deplete_requests_from_stdin(luts: &Vec<BTreeMap<Oid, Capsule>>) -> Result<(), Error> {
    let all_oids = lut::commit_oids_table(luts);
    let mut commits = Vec::new();

    let stdin = stdin();
    let stdout = stdout();

    let read = BufReader::new(stdin.lock());
    let mut out = stdout.lock();
    let mut obuf = String::new();

    eprintln!("Waiting for input...");
    for hexsha in read.lines().filter_map(Result::ok) {
        let oid = Oid::from_str(&hexsha)?;

        commits.clear();
        lut::commits_by_blob(&oid, luts, &all_oids, &mut commits);

        obuf.clear();
        let len = commits.len();
        for (cid, commit_oid) in commits.iter().enumerate() {
            use std::fmt::Write;
            write!(obuf, "{}", commit_oid)?;
            if cid + 1 < len {
                obuf.push(' ');
            }
        }
        obuf.push('\n');

        write!(out, "{}", obuf)?;
        out.flush()?;
    }
    Ok(())
}

mod find {
    use failure::Error;
    use std::path::Path;
    use std::collections::BTreeMap;
    use Capsule;
    use git2::Oid;
    use lut;

    pub fn commit(_tree: &Path, luts: Vec<BTreeMap<Oid, Capsule>>) -> Result<(), Error> {
        let _all_oids = lut::commit_oids_table(&luts);
        //        let mut commits = Vec::new();

        //        let mut blobs = Vec
        unimplemented!();
    }
}

fn run(opts: Options) -> Result<(), Error> {
    let tree = opts.tree.clone();
    let luts = lut::build(opts)?;
    match tree {
        None => deplete_requests_from_stdin(&luts),
        Some(tree) => find::commit(&tree, luts),
    }
}

fn main() {
    let opts = Options::from_args();
    ok_or_exit(run(opts));
}
