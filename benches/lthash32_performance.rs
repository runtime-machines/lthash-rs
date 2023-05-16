use lthash_rs::LtHash;
use std::{iter, time::Duration};

use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, WallTime},
    BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};

fn gen_bytes(length: usize) -> Box<[u8]> {
    let elem: Vec<_> = b"hello-world"
        .iter()
        .copied()
        .cycle()
        .take(length)
        .collect();
    elem.into_boxed_slice()
}

trait MeasurementName: Measurement {
    fn name() -> &'static str;
}

impl MeasurementName for WallTime {
    fn name() -> &'static str {
        "wall-time"
    }
}

fn extend<M: MeasurementName>(c: &mut Criterion<M>) {
    let name = M::name();

    let mut extend_benchmark_group =
        c.benchmark_group(format!("extend_{name}_100"));

    extend_benchmark_group
        .sample_size(1000)
        .measurement_time(Duration::from_secs(7));

    extend_group(extend_benchmark_group, 100);

    let mut extend_benchmark_group =
        c.benchmark_group(format!("extend_{name}_10000"));

    extend_benchmark_group
        .sample_size(100)
        .measurement_time(Duration::from_secs(12));

    extend_group(extend_benchmark_group, 10000);

    let mut extend_benchmark_group =
        c.benchmark_group(format!("extend_{name}_100000"));

    extend_benchmark_group
        .sample_size(20)
        .measurement_time(Duration::from_secs(60));

    extend_group(extend_benchmark_group, 100000);
}

fn extend_group<M: Measurement>(mut group: BenchmarkGroup<M>, objects: usize) {
    for bytes in (1..=20)
        .map(|it| it * 50)
        .chain([1500, 2000].iter().copied())
    {
        let input: Vec<_> =
            iter::repeat(gen_bytes(bytes)).take(objects).collect();

        group
            .throughput(Throughput::Bytes(bytes as u64 * objects as u64))
            .bench_with_input(
                BenchmarkId::from_parameter(bytes),
                &input,
                |b, input| {
                    b.iter(|| {
                        let mut test =
                            lthash_rs::LtHash32::<sha3::Shake128>::new();
                        test.extend(input)
                    })
                },
            );
    }
}

fn insert<M: MeasurementName>(c: &mut Criterion<M>) {
    let mut insert_benchmark_group =
        c.benchmark_group(format!("insert_{}", M::name()));

    insert_benchmark_group
        .sample_size(5000)
        .measurement_time(Duration::from_secs(6));

    for bytes in (1..=20)
        .map(|it| it * 50)
        .chain([1500, 2000].iter().copied())
    {
        let input = gen_bytes(bytes);

        insert_benchmark_group
            .throughput(Throughput::Bytes(bytes as u64))
            .bench_with_input(
                BenchmarkId::new("shake128", bytes),
                &input,
                |b, input| {
                    b.iter(|| {
                        let mut test =
                            lthash_rs::LtHash32::<sha3::Shake128>::new();
                        test.insert(input)
                    })
                },
            );

        insert_benchmark_group
            .throughput(Throughput::Bytes(bytes as u64))
            .bench_with_input(
                BenchmarkId::new("shake256", bytes),
                &input,
                |b, input| {
                    b.iter(|| {
                        let mut test =
                            lthash_rs::LtHash32::<sha3::Shake256>::new();
                        test.insert(input)
                    })
                },
            );
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(lthash_rs::LtHash32::<sha3::Shake128>::new)
    });
}

criterion_group!(wall, criterion_benchmark, insert, extend);

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] {
        use criterion_cycles_per_byte::CyclesPerByte;

        impl MeasurementName for CyclesPerByte {
            fn name() -> &'static str {
                "cycles-per-byte"
            }
        }

        criterion_group!(
            name = cycles;
            config = Criterion::default().with_measurement(CyclesPerByte);
            targets = insert, extend
        );

    } else {
        criterion_group!(
            name = cycles;
            config = Criterion::default();
            targets = insert, extend
        );
    }
}

criterion_main!(wall, cycles);
