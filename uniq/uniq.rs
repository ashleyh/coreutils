#[crate_id(name="uniq", vers="1.0.0", author="ashleyh")];

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Ashley Hewson <gh@ashleyh.eu>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use std::io::{stdin, stdout};
use std::io::{Writer, Buffer};
use std::io::buffered::{BufferedReader, BufferedWriter};
use std::iter::Peekable;

pub struct LineReader<R> {
  priv reader: R,
}

impl<R: Buffer> Iterator<~[u8]> for LineReader<R> {
  fn next(&mut self) -> Option<~[u8]> {
    self.reader.read_line().map( |line| { line.into_bytes() })
  }
}

fn is_dup(a: &~[u8], b: &~[u8]) -> bool {
  return a.eq(b);
}

/// Advances `it` until `pred` would return `false` on the next element
fn skip<A, T: Iterator<A>>(it: &mut Peekable<A, T>, pred: |&A| -> bool) {
  loop {
    match it.peek() {
      Some(x) if pred(x) => {},
      _ => break,
    };
    it.next();
  }
}

fn main() {
  let mut r = LineReader {
    reader: BufferedReader::new(stdin()),
  }.peekable();
  let mut w = BufferedWriter::new(stdout());
  loop {
    let first_line = match r.next() {
      Some(first_line) => first_line,
      None => break,
    };
    w.write(first_line);
    skip(&mut r, |next_line| is_dup(&first_line, next_line));
  }
  w.flush();
}
