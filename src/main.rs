extern crate failure;
extern crate failure_tools;
extern crate git2;
extern crate indicatif;
#[macro_use]
extern crate structopt;
extern crate crossbeam;
extern crate fixedbitset;
extern crate num_cpus;
extern crate walkdir;

use failure_tools::ok_or_exit;
use std::path::PathBuf;
use git2::{ObjectType, Oid};
use structopt::StructOpt;

mod lut;
mod cli;

fn main() {
    let opts = Options::from_args();
    ok_or_exit(cli::run(opts));
}

#[derive(Clone)]
pub enum Capsule {
    Normal(Vec<Oid>),
    Compact(Vec<usize>),
}

#[derive(Default)]
pub struct Stack {
    indices: Vec<usize>,
    oids: Vec<Oid>,
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
}
