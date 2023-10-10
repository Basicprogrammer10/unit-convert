use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use unit_convert::dimension::{expander::Expander, tokenizer::Tokenizer, tree::Treeifyer};

macro_rules! bench_parse_components {
    ($c:expr, $unit:literal) => {
        $c.bench_function(concat!("tokenize `", $unit, "`"), |b| {
            b.iter(|| Tokenizer::tokenize(black_box($unit)).unwrap())
        });

        let tokens = Tokenizer::tokenize($unit).unwrap();
        $c.bench_function(concat!("tree `", $unit, "`"), |b| {
            b.iter_batched(
                || tokens.clone(),
                |tokens| Treeifyer::treeify(black_box(tokens)),
                BatchSize::SmallInput,
            )
        });

        let tree = Treeifyer::treeify(tokens).unwrap();
        $c.bench_function(concat!("expand `", $unit, "`"), |b| {
            b.iter_batched(
                || tree.clone(),
                |tree| Expander::expand(black_box(tree)),
                BatchSize::SmallInput,
            )
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_parse_components!(c, "m/s^2");
    // bench_parse_components!(c, "m/s/s");
    // bench_parse_components!(c, "m/(s*s)");

    c.bench_function("lookup_unit", |b| {
        b.iter(|| unit_convert::units::find_unit(black_box("m")))
    });
    c.bench_function("lookup_unit_shorthand", |b| {
        b.iter(|| unit_convert::units::find_unit(black_box("Wh")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
