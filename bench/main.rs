mod utils;

use criterion::*;
use oasysdb::prelude::*;
use utils::*;

extern crate nalgebra as na;

use na::{distance, ComplexField, DVector, OPoint, Point};

fn build_collection(path: &str) -> Collection {
    let records = get_records(path).unwrap();
    let config = Config::default();
    Collection::build(&config, &records).unwrap()
}

fn bench_search_collection(criterion: &mut Criterion) {
    let id = "Search collection";

    // Download the dataset.
    download_siftsmall().unwrap();

    // Load the query data.
    let query_path = "data/siftsmall/siftsmall_query.fvecs";
    let query_data = read_vectors(query_path).unwrap();
    let vector: Vector = query_data[0].clone().into();

    // Create the collection.
    let base_path = "data/siftsmall/siftsmall_base.fvecs";
    let collection = build_collection(base_path);

    // Benchmark the search speed.
    let routine = || {
        black_box(collection.search(&vector, 10).unwrap());
    };

    criterion.bench_function(id, |bencher| bencher.iter(routine));
}

fn bench_calculation_10k(criterion: &mut Criterion) {
    let n = Vector::random(10000);
    let v = Vector::random(10000);
    let dist_func = Distance::Euclidean;

    criterion.bench_function("bench_calculation_10k", |b| {
        b.iter(|| dist_func.calculate(&n, &v))
    });
}
fn bench_calculation_100k(criterion: &mut Criterion) {
    let n = Vector::random(100000);
    let v = Vector::random(100000);
    let dist_func = Distance::Euclidean;

    criterion.bench_function("bench_calculation_100k", |b| {
        b.iter(|| dist_func.calculate(&n, &v))
    });
}
fn bench_calculation_1000k(criterion: &mut Criterion) {
    let n = Vector::random(1000000);
    let v = Vector::random(1000000);
    let dist_func = Distance::Euclidean;

    criterion.bench_function("bench_calculation_1000k", |b| {
        b.iter(|| dist_func.calculate(&n, &v))
    });
}

fn bench_calculation_1b(criterion: &mut Criterion) {
    let n = Vector::random(10000000);
    let v = Vector::random(10000000);
    let dist_func = Distance::Euclidean;

    criterion.bench_function("bench_calculation_1b", |b| {
        b.iter(|| dist_func.calculate(&n, &v))
    });
}

fn bench_calculation_100b(criterion: &mut Criterion) {
    let n = Vector::random(1000000000);
    let v = Vector::random(1000000000);
    let dist_func = Distance::Euclidean;

    criterion.bench_function("bench_calculation_100b", |b| {
        b.iter(|| dist_func.calculate(&n, &v))
    });
}

fn bench_calculation_10k_acc(criterion: &mut Criterion) {
    let n = Vector::random(10000);
    let v = Vector::random(10000);
    let dv1 = DVector::from(n.0);
    let dv2 = DVector::from(v.0);

    criterion.bench_function("bench_calculation_10k_acc", |b| {
        b.iter(|| {
            let rst = dv1.metric_distance(&dv2) + 0.1;
            rst
        })
    });
}

fn bench_calculation_100k_acc(criterion: &mut Criterion) {
    let n = Vector::random(100000);
    let v = Vector::random(100000);
    let dv1 = DVector::from(n.0);
    let dv2 = DVector::from(v.0);

    criterion.bench_function("bench_calculation_100k_acc", |b| {
        b.iter(|| {
            let rst = dv1.metric_distance(&dv2) + 0.1;
            rst
        })
    });
}

fn bench_calculation_1000k_acc(criterion: &mut Criterion) {
    let n = Vector::random(1000000);
    let v = Vector::random(1000000);
    let dv1 = DVector::from(n.0);
    let dv2 = DVector::from(v.0);

    criterion.bench_function("bench_calculation_1000k_acc", |b| {
        b.iter(|| {
            let rst = dv1.metric_distance(&dv2);
        })
    });
}

fn bench_calculation_1b_acc(criterion: &mut Criterion) {
    let n = Vector::random(10000000);
    let v = Vector::random(10000000);
    let dv1 = DVector::from(n.0);
    let dv2 = DVector::from(v.0);

    criterion.bench_function("bench_calculation_1b_acc", |b| {
        b.iter(|| {
            let rst = dv1.metric_distance(&dv2)+0.1;
            rst
        })
    });
}

fn bench_calculation_100b_acc(criterion: &mut Criterion) {
    let n = Vector::random(1000000000);
    let v = Vector::random(1000000000);
    let dv1 = DVector::from(n.0);
    let dv2 = DVector::from(v.0);

    criterion.bench_function("bench_calculation_100b_acc", |b| {
        b.iter(|| {
            let rst = dv1.metric_distance(&dv2) + 1.0;
            rst
        })
    });
}

criterion_group!(
    bench,
    bench_calculation_10k,
    bench_calculation_10k_acc,
    bench_calculation_100k,
    bench_calculation_100k_acc,
    bench_calculation_1000k,
    bench_calculation_1000k_acc,
    bench_calculation_1b,
    bench_calculation_1b_acc,
    // bench_calculation_100b,
    bench_calculation_100b_acc,
);
// criterion_group!(bench, bench_calculation_100k);
// criterion_group!(bench, bench_calculation_1000k);
// criterion_group!(bench, bench_calculation_1b);
criterion_main!(bench);
