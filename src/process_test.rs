use im::vector;
use std::sync::Arc;

use crate::expressions::RuntimeExpression::{self, List, Number};
use crate::process::Process::{Complete, Running};
use crate::process::*;

fn make_process(a: u8, b: u8, c: u8) -> Process<RuntimeExpression> {
    Running(Arc::new(move || {
        Running(Arc::new(move || {
            Running(Arc::new(move || {
                Complete(List(vector![Number(a), Number(b), Number(c)]))
            }))
        }))
    }))
}

#[test]
fn test_process_by_steps() {
    let actual = make_process(1, 2, 3).step().step().step();

    assert!(actual.is_complete());
    assert!(!actual.is_running());

    let expected = List(vector![Number(1), Number(2), Number(3)]);

    assert_eq!(expected, actual.result().unwrap());
}

#[test]
fn test_process_to_completion() {
    let actual = make_process(1, 2, 3).run_until_complete();

    let expected = List(vector![Number(1), Number(2), Number(3)]);
    assert_eq!(expected, actual);
}

#[test]
fn test_round_robin_processes_to_completion() {
    let actual = Process::round_robin(vector![
        make_process(1, 2, 3),
        make_process(4, 5, 6),
        make_process(7, 8, 9),
    ]);

    let expected = vector![
        List(vector![Number(1), Number(2), Number(3)]),
        List(vector![Number(4), Number(5), Number(6)]),
        List(vector![Number(7), Number(8), Number(9)]),
    ];

    assert_eq!(expected, actual);
}
#[test]
fn test_round_robin_processes_with_complete() {
    let actual = Process::round_robin(vector![
        make_process(1, 2, 3),
        Complete(List(vector![Number(4), Number(5), Number(6)])),
        make_process(7, 8, 9),
    ]);

    let expected = vector![
        List(vector![Number(4), Number(5), Number(6)]),
        List(vector![Number(1), Number(2), Number(3)]),
        List(vector![Number(7), Number(8), Number(9)]),
    ];

    assert_eq!(expected, actual);
}

#[test]
fn test_complete_sequence() {
    let input = vector![
        Complete(Number(1)),
        Complete(Number(2)),
        Complete(Number(3))
    ];

    let process = Process::run_in_sequence(input);

    let actual = process.run_until_complete();

    let expected = vector![Number(1), Number(2), Number(3)];

    assert_eq!(expected, actual);
}

#[test]
fn test_running_sequence() {
    let input = vector![
        make_process(1, 2, 3),
        make_process(4, 5, 6),
        make_process(7, 8, 9),
    ];

    let process = Process::run_in_sequence(input);

    let actual = process.run_until_complete();

    let expected = vector![
        List(vector![Number(1), Number(2), Number(3)]),
        List(vector![Number(4), Number(5), Number(6)]),
        List(vector![Number(7), Number(8), Number(9)]),
    ];

    assert_eq!(expected, actual);
}

#[test]
fn test_and_then() {
    let process = Running(Arc::new(|| Complete(1))).and_then(Arc::new(|n| Complete((n, 2))));
    assert_eq!((1, 2), process.run_until_complete());
}
