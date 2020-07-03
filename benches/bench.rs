use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn identity(set: HashSet<Vec<u8>>) -> HashSet<Vec<u8>> {
    set
}

fn identity_as_iterator(set: HashSet<Vec<u8>>) -> impl Iterator<Item = Vec<u8>> {
    set.into_iter()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("into_iterator_collect_optimization");

    let set = (0..42)
        .map(|_| {
            thread_rng()
                .sample_iter(&Standard)
                .take(42)
                .collect::<Vec<u8>>()
        })
        .collect::<HashSet<Vec<u8>>>();

    group.bench_function("identity", |b| {
        assert_eq!(42, set.len());
        let mut set = Some(set.clone());

        b.iter(move || set = Some(identity(set.take().unwrap())));
    });

    group.bench_function("identity_as_iterator", |b| {
        assert_eq!(42, set.len());
        let mut set = Some(set.clone());

        b.iter(move || set = Some(identity_as_iterator(set.take().unwrap()).collect()));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
