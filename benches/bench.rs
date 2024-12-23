use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use questionabletenthash::tenthash;
use tenthash::{hash, TentHasher};

//----

fn tent_hash_single_author(c: &mut Criterion) {
    let benches = [
        ("10b_message", 10),       // 10-byte input.
        ("100b_message", 100),     // 100-byte input.
        ("1kb_message", 1000),     // 1-kilobyte input.
        ("10kb_message", 10000),   // 10-kilobyte input.
        ("100kb_message", 100000), // 100-kilobyte input.
        ("1mb_message", 1000000),  // 1-megabyte input.
    ];

    let mut group = c.benchmark_group("tent_hash_nathan");

    for (name, data_size) in benches.iter() {
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(*data_size)
            .collect();
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let _ = hash(&data);
            })
        });
    }
}
//----
fn tent_hash_single_zaq(c: &mut Criterion) {
    let benches = [
        ("10b_message", 10),       // 10-byte input.
        ("100b_message", 100),     // 100-byte input.
        ("1kb_message", 1000),     // 1-kilobyte input.
        ("10kb_message", 10000),   // 10-kilobyte input.
        ("100kb_message", 100000), // 100-kilobyte input.
        ("1mb_message", 1000000),  // 1-megabyte input.
    ];

    let mut group = c.benchmark_group("tent_hash_zaq");

    for (name, data_size) in benches.iter() {
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(*data_size)
            .collect();
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let _ = tenthash(&data);
            })
        });
    }
}
criterion_group!(benches, tent_hash_single_author, tent_hash_single_zaq);
criterion_main!(benches);
