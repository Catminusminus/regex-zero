use criterion::{criterion_group, criterion_main, Criterion};
use regex_zero::{do_matching, do_matching_with_cache};
use std::time::Duration;

const INPUTS: &[(&str, &str, &str)] = &[
    ("n=2", "a?a?aa", "aa"),
    ("n=4", "a?a?a?a?aaaa", "aaaa"),
    ("n=6", "a?a?a?a?a?a?aaaaaa", "aaaaaa"),
    ("n=8", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    ("n=10", "a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa", "aaaaaaaaaa"),
];

const REDOS_INPUTS: &[(&str, &str, &str)] = &[
    ("i=22", "q(i+|t)+a", "qiiiiiiiiiiiiiiiiiiiiiite"),
    ("i=23", "q(i+|t)+a", "qiiiiiiiiiiiiiiiiiiiiiiite"),
    ("i=24", "q(i+|t)+a", "qiiiiiiiiiiiiiiiiiiiiiiiite"),
];

fn without_cache(c: &mut Criterion) {
    let mut g = c.benchmark_group("Without Cache");
    g.measurement_time(Duration::from_secs(80));

    for i in REDOS_INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| do_matching(args.0, args.1, true))
        });
    }
}

fn with_cache(c: &mut Criterion) {
    let mut g = c.benchmark_group("With Cache");
    g.measurement_time(Duration::from_secs(12));

    for i in REDOS_INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| do_matching_with_cache(args.0, args.1, true))
        });
    }
}

criterion_group!(benches, without_cache, with_cache);
criterion_main!(benches);
