use criterion::{criterion_group, criterion_main, Criterion};
use rsff::balloon::Balloon;
use rsff::*;

pub fn create_document() -> Document {
    Document::default()
}

pub fn document_benches(c: &mut Criterion) {
    let mut doc = Document::default();

    let mut bln = Balloon::default();
    bln.tl_content.push(String::from("asdfasdf"));
    bln.tl_content.push(String::from("asdfasdf"));
    bln.tl_content.push(String::from("asdfasdf"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.add_image("jpg".into(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    for _ in 0..100 {
        doc.balloons.push(bln.clone());
    }

    let mut g = c.benchmark_group("Document Serialization");

    g.sample_size(10_000);
    g.bench_function("Json", |b| b.iter(|| serde_json::to_string(&doc).unwrap()));
}

criterion_group!(benches, document_benches);
criterion_main!(benches);
