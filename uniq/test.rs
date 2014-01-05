use std::{run, str};

#[test]
fn test_stdin_simple() {
  let mut prog = run::Process::new("build/uniq", [], run::ProcessOptions::new()).unwrap();
  prog.input().write(bytes!("a\nb\nb\nb\nc"));
  prog.close_input();
  let out = str::from_utf8_owned(prog.finish_with_output().output);
  assert_eq!(out, ~"a\nb\nc")
}
