use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use rust_project_structure_demo::greet;

fn bench_greet(c: &mut Criterion) {
    c.bench_function("greet", |b| b.iter(|| greet(black_box("Alice"))));
}

criterion_group!(benches, bench_greet);
criterion_main!(benches);
