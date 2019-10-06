#[macro_use]
extern crate clap;
extern crate procfs;

use std::time::{Instant, Duration};
use clap::{App, Arg};
use std::io::{stdout, Write, BufWriter};
use std::thread::sleep;
use std::collections::HashMap;

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
        .arg(Arg::with_name("diff")
            .help("diff mode")
            .short("m")
            .long("mode")
            .takes_value(true)
        )
        .get_matches();

    let duration = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_DURATION);
    let tick_milliseconds = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_TICK);
    let diff_flag = value_t!(matches, "diff", bool).unwrap_or(false);

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

        let mut plist = HashMap::new();

        for p in &all_procs {
            if let Ok(pexec) = p.exe() {
                if diff_flag && !plist.contains_key(pexec.to_str().unwrap()) {
                    let pexec_name = pexec.to_str().unwrap().to_string();
                    plist.insert(pexec_name, p.pid());
                }

                if !diff_flag {
                    writeln!(out, "{}", pexec.display()).unwrap();
                }
            }
        }

        if diff_flag {
            for (exec_name, _) in &plist {
                writeln!(out, "{}", exec_name).unwrap();
            }
            plist.clear();
        }

        writeln!(out, "\n").unwrap();


        if first.elapsed() > limit {
            done = true;
        }

        sleep(Duration::from_millis(tick_milliseconds));
    }
}
