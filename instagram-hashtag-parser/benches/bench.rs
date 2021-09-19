#[macro_use]
extern crate criterion;

use criterion::Criterion;
use csv::Reader;
use instagram_hashtag_parser::{hashtags_with_crate_hashtag, hashtags_with_regex};
use once_cell::sync::Lazy;
use serde::Deserialize;

static CAPTIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let data = include_str!("../tests/files/captions.csv");
    let mut rdr = Reader::from_reader(data.as_bytes());
    #[derive(Deserialize)]
    struct Row {
        caption: String,
    }

    let mut captions = vec![];
    for row in rdr.deserialize::<Row>().flatten() {
        captions.push(row.caption);
    }
    captions
});

fn with_regex(c: &mut Criterion) {
    c.bench_function("with_regex", move |b| {
        b.iter(|| {
            CAPTIONS
                .iter()
                .map(|x| hashtags_with_regex(x))
                .collect::<Vec<_>>()
        });
    });
}

fn with_crate_hashtag(c: &mut Criterion) {
    c.bench_function("with_crate_hashtag", move |b| {
        b.iter(|| {
            CAPTIONS
                .iter()
                .map(|x| hashtags_with_crate_hashtag(x))
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, with_regex, with_crate_hashtag,);
criterion_main!(benches);
