use criterion::{Criterion, black_box, criterion_group, criterion_main};
use spatial_decomposition::*;

fn kmr_digitize_rayon(n_digits: usize, n_subdomains: usize) -> Result<SortedDigits<f64, usize>> {
    use rayon::prelude::*;
    let domain = Rectangle {
        min: [0.0; 2],
        max: [100.0; 2],
    };
    let decomposed_domain = kmr_decompose(&domain, n_subdomains.try_into().unwrap());
    let digits = kmr_decompose(&domain, n_digits.try_into().unwrap());
    let res = digits
        .into_par_iter()
        .enumerate()
        .map(|(digit, rect)| {
            let index = kmr_digitize_1_single(&decomposed_domain, &rect)?;
            Ok((index, digit, rect))
        })
        .fold(
            std::collections::BTreeMap::new,
            |mut acc: std::collections::BTreeMap<usize, Vec<(usize, Rectangle<f64>)>>,
             b: Result<(usize, usize, Rectangle<f64>)>| {
                let (key, digit, rect) = b.unwrap();
                acc.entry(key).or_default().push((digit, rect));
                acc
            },
        )
        .reduce(std::collections::BTreeMap::new, |mut acc, b| {
            for (key, value) in b.into_iter() {
                acc.entry(key).or_default().extend(value);
            }
            acc
        });

    let res = decomposed_domain
        .into_iter()
        .zip(res)
        .map(|(rect, (_, values))| (rect, values))
        .collect();
    Ok(res)
}

fn kmr_digitize_serial(n_digits: usize, n_subdomains: usize) -> Result<SortedDigits<f64, usize>> {
    let domain = Rectangle {
        min: [0.0; 2],
        max: [100.0; 2],
    };
    let digits = kmr_decompose(&domain, n_digits.try_into().unwrap())
        .into_iter()
        .enumerate();
    kmr_digitize_1(&domain, n_subdomains.try_into().unwrap(), digits)
}

pub fn kong_mount_roscoe(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build_global()
        .unwrap();
    use criterion::BenchmarkId;
    for n in 1..20usize {
        let n_subdomains = n * 10;
        let id = BenchmarkId::new("kmr_decompose_square", n_subdomains);
        c.bench_with_input(id, &n_subdomains, |b, n_subdomains| {
            b.iter(|| {
                let domain = Rectangle {
                    min: [0.0; 2],
                    max: [100.0; 2],
                };
                kmr_decompose(
                    black_box(&domain),
                    black_box((*n_subdomains).try_into().unwrap()),
                );
            });
        });
    }

    let n_subdomains = 20;
    for n in 1..5 {
        let n_digits = n * 1_000;
        c.bench_with_input(
            BenchmarkId::new(
                "kmr_digitize_square_rayon",
                format!("{}-{}", n_digits, n_subdomains),
            ),
            &(n_digits, n_subdomains),
            |b, input| b.iter(|| kmr_digitize_rayon(black_box(input.0), black_box(input.1))),
        );
    }
    for n in 1..5 {
        let n_digits = n * 1000;
        c.bench_with_input(
            BenchmarkId::new(
                "kmr_digitize_square_serial",
                format!("{}-{}", n_digits, n_subdomains),
            ),
            &(n_digits, n_subdomains),
            |b, input| b.iter(|| kmr_digitize_serial(black_box(input.0), black_box(input.1))),
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = kong_mount_roscoe
}
criterion_main!(benches);
