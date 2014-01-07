#[crate_id(name="uniq", vers="1.0.0", author="ashleyh")];

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Ashley Hewson <gh@ashleyh.eu>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern mod extra;
use std::io::{stdin, stdout, stderr};
use std::io::{Writer, Buffer};
use std::io::buffered::{BufferedReader, BufferedWriter};
use std::iter::Peekable;
use std::os;
use extra::getopts::groups;
use std::from_str::from_str;

pub struct LineReader<R> {
    priv reader: R,
}

impl<R: Buffer> Iterator<~[u8]> for LineReader<R> {
    fn next(&mut self) -> Option<~[u8]> {
        self.reader.read_line().map( |line| { line.into_bytes() })
    }
}

struct CompareOpts {
    skip_chars: uint,
}

impl CompareOpts {
    fn nub<'a>(&self, s: &'a [u8]) -> &'a [u8] {
        s.slice_from(self.skip_chars)
    }
    fn compare(&self, a: &[u8], b: &[u8]) -> bool {
        let a_nub = self.nub(a);
        let b_nub = self.nub(b);
        a_nub.eq(&b_nub)
    }
}

struct Group<'a, R> {
    first_line: &'a [u8],
    p: &'a mut Peekable<~[u8], R>,
    compare_opts: CompareOpts,
}

impl<'a, R: Buffer> Group<'a, LineReader<R>> {
    fn has_more(&self) -> bool {
        match self.p.peek() {
            Some(next_line) => self.compare_opts.compare(self.first_line, *next_line),
            _ => false,
        }
    }

    fn each_line(&self, f: |~[u8]|) {
        loop {
            if self.has_more() {
                f(self.p.next().unwrap());
            } else {
                break;
            }
        }
    }
}

fn each_group<R: Buffer>(r: LineReader<R>, c: CompareOpts, f: |&Group<LineReader<R>>|) {
    let mut p = r.peekable();
    loop {
        let first_line = match p.next() {
            Some(first_line) => first_line,
            _ => break,
        };
        let group = Group {
            first_line: first_line,
            p: &mut p,
            compare_opts: c,
        };
        f(&group);
        group.each_line(|_| {});
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();
    let opts = ~[
        groups::optflag("u", "unique", "print only unique lines"),
        groups::optflag("c", "count", "print the number of lines in each group"),
        groups::optflag("d", "repeated", "print only repeated lines"),
        groups::optopt("s", "skip-chars", "ignore first N chars when comparing lines", "N"),
    ];

    let matches = match groups::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => {
            writeln!(&mut stderr(),
                "{}: {}",
                program, f.to_err_msg());
            os::set_exit_status(1);
            return;
        },
    };

    let c = CompareOpts {
        skip_chars: matches.opt_str("s")
                           .and_then(|s| from_str::<uint>(s))
                           .unwrap_or(0),
    };

    let r = LineReader {
        reader: BufferedReader::new(stdin()),
    };
    let mut w = BufferedWriter::new(stdout());

    if matches.opt_present("d") {
        each_group(r, c, |group| {
            if group.has_more() {
                w.write(group.first_line);
            }
        })
    } else if matches.opt_present("c") {
        each_group(r, c, |group| {
            let mut n = 1;
            group.each_line(|_| {n += 1;});
            write!(&mut w, "{:7d} ", n);
            w.write(group.first_line);
        })
    } else if matches.opt_present("u") {
        each_group(r, c, |group| {
            if !group.has_more() {
                w.write(group.first_line);
            }
        })
    } else {
        each_group(r, c, |group| {
            w.write(group.first_line);
        });
    }
    w.flush();
}
