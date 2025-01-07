fn fit_decode() {
    let data: &[u8] = include_bytes!("../tests/fixtures/sample_mulitple_header.fit");
    let fit_data = fitparser::from_bytes(data).unwrap();
    assert_eq!(fit_data.len(), 3023);
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("Fit decoding", |b| b.iter(|| fit_decode()));
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);