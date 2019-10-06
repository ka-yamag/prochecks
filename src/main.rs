#[macro_use]
extern crate clap;
extern crate procfs;

use std::time::{Instant, Duration};
use clap::{App, Arg};
use std::io::{stdout, Write, BufWriter};
use std::thread::sleep;

const DEFAULT_DURATION: u64 = 5;
const DEFAULT_TICK: u64 = 200;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("duration")
            .help("duration time")
            .short("d")
            .long("duration")
            .takes_value(true)
        )
        .arg(Arg::with_name("tick")
            .help("ticker milliseconds")
            .short("t")
            .long("tick")
            .takes_value(true)
        )
        .get_matches();

    let duration = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_DURATION);
    let tick_milliseconds = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_TICK);
    println!("{}", tick_milliseconds);

    let first = Instant::now();
    let limit = Duration::from_secs(duration);
    let mut done = false;

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    while !done {

        let all_procs = procfs::all_processes();

        writeln!(out, "elasped time: {}.{}",
            first.elapsed().as_secs(),
            first.elapsed().subsec_millis()
            ).unwrap();

        for p in &all_procs {
            if let Ok(pexec) = p.exe() {
                writeln!(out, "{}", pexec.display()).unwrap();
            }
        }
        writeln!(out, "\n").unwrap();

        if first.elapsed() > limit {
            done = true;
        }

        sleep(Duration::from_millis(tick_milliseconds));
    }
}
