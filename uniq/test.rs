use std::{run, str};
use std::vec;

fn run_prog(args: &[~str]) -> run::Process {
    run::Process::new(
        "build/uniq",
        vec::append(~[~"build/uniq"], args),
        run::ProcessOptions::new()
    ).unwrap()
}

#[test]
fn test_stdin_simple() {
    let mut prog = run_prog([]);
    prog.input().write(bytes!("a\nb\nb\nb\nc"));
    prog.close_input();
    let out = str::from_utf8_owned(prog.finish_with_output().output);
    assert_eq!(out, ~"a\nb\nc")
}

#[test]
fn test_stdin_unique() {
    let mut prog = run_prog([~"-u"]);
    prog.input().write(bytes!("a\nb\nb\nb\nc"));
    prog.close_input();
    let out = str::from_utf8_owned(prog.finish_with_output().output);
    assert_eq!(out, ~"a\nc")
}

#[test]
fn test_stdin_count() {
    let mut prog = run_prog([~"-c"]);
    prog.input().write(bytes!("a\nb\nb\nb\nc"));
    prog.close_input();
    let out = str::from_utf8_owned(prog.finish_with_output().output);
    assert_eq!(out, ~"      1 a\n      3 b\n      1 c")
}

#[test]
fn test_stdin_repeated() {
    let mut prog = run_prog([~"-d"]);
    prog.input().write(bytes!("a\nb\nb\nb\nc"));
    prog.close_input();
    let out = str::from_utf8_owned(prog.finish_with_output().output);
    assert_eq!(out, ~"b\n")
}

#[test]
fn test_stdin_skip_char() {
    let mut prog = run_prog([~"-s1"]);
    // XXX: doesn't match GNU uniq if no newline at end of file
    prog.input().write(bytes!("aa\nba\nab\nbb\n"));
    prog.close_input();
    let out = str::from_utf8_owned(prog.finish_with_output().output);
    assert_eq!(out, ~"aa\nab\n")
}
