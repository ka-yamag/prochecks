#[macro_use]
extern crate clap;
extern crate procfs;

use std::time::{Instant, Duration};
use clap::{App, Arg};
use std::io::{stdout, Write, BufWriter};
use std::thread::sleep;
use std::collections::HashMap;
use chrono::{Local, DateTime};

const DEFAULT_DURATION: u64 = 5;
const DEFAULT_TICK: u64 = 200;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("duration")
            .help("duration time [s]")
            .short("d")
            .long("duration")
            .takes_value(true)
        )
        .arg(Arg::with_name("tick")
            .help("ticker [ms]")
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
        .arg(Arg::with_name("target")
            .help("specify target")
            .short("s")
            .long("target")
            .takes_value(true)
        )
        .get_matches();

    let duration = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_DURATION);
    let tick_milliseconds = value_t!(matches, "duration", u64).unwrap_or(DEFAULT_TICK);
    let diff_flag = value_t!(matches, "diff", bool).unwrap_or(false);
    let target_name = value_t!(matches, "target", String).unwrap_or(String::from(""));

    let first = Instant::now();
    let limit = Duration::from_secs(duration);
    let mut done = false;

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    while !done {
        let all_procs = procfs::all_processes();

        let local_datetime: DateTime<Local> = Local::now();
        writeln!(out, "local sys time: {}", local_datetime).unwrap();

        writeln!(out, "elasped time: {}.{}",
            first.elapsed().as_secs(),
            first.elapsed().subsec_millis()
            ).unwrap();

        let mut plist = HashMap::new();

        for p in &all_procs {
            if let Ok(pexec) = p.exe() {

                if target_name != "" {
                    let pexec_name = pexec.to_str().unwrap().to_string();

                    if pexec_name.contains(&target_name) {
                        let pp = procfs::Process::new(p.stat.ppid).unwrap();

                        if let Ok(pp_cmdline) = pp.cmdline() {
                            writeln!(out, "target {} => parenet process : {}", target_name, pp_cmdline.iter().cloned().split_whitespace().collect::<String>()).unwrap();
                        }
                    }
                } else {
                    if let Ok(cmdline) = p.cmdline() {
                        if diff_flag && !plist.contains_key(pexec.to_str().unwrap()) {
                            let pexec_name = pexec.to_str().unwrap().to_string();
                            plist.insert(pexec_name, cmdline.iter().cloned().split_whitespace().collect::<String>());
                        }

                        if !diff_flag {
                            writeln!(out, "{}", cmdline.iter().cloned().collect::<String>()).unwrap();
                        }
                    }
                }

            }
        }

        if diff_flag {
            for (_, cmd) in &plist {
                writeln!(out, "{}", cmd).unwrap();
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
