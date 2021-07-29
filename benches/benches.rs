use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{Cursor, Read};
use serde_dummy::*;

fn serialize_json<R: Read>(reader: R) {
    let _: serde_json::Value = serde_json::from_reader(reader).unwrap();
}

fn validate_json<R: Read>(reader: R) {
    let _: Any = serde_json::from_reader(reader).unwrap();
}

static BYTES: &'static [u8] = include_bytes!("../small.json");

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("serialize", |b| {
        b.iter(|| {
            let rdr = Cursor::new(&BYTES);
            serialize_json(rdr);
        })
    });

    c.bench_function("validate", |b| {
        b.iter(|| {
            let rdr = Cursor::new(&BYTES);
            validate_json(rdr);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
