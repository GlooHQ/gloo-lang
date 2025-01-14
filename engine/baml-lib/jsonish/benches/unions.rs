use criterion::Criterion;
use internal_baml_jinja::types::Builder;
use jsonish::from_str;
use baml_types::FieldType;

pub fn bench_unions(c: &mut Criterion) {
    let mut group = c.benchmark_group("unions");

    let target = FieldType::union(vec![
        FieldType::Class("TextContent".to_string()),
        FieldType::Class("ImageContent".to_string()),
        FieldType::Class("VideoContent".to_string()),
    ]);
    let ir = jsonish::helpers::load_test_ir(UNION_SCHEMA);
    let target = jsonish::helpers::render_output_format(&ir, &target, &Default::default()).unwrap();

    
    let of = Builder::new(target.clone()).build();
    
    group.bench_function("text_content", |b| b.iter(|| from_str(
        &of,
        &target,
        r#"{"text": "Hello World"}"#,
        false,
    )));

    group.bench_function("image_content", |b| b.iter(|| from_str(
        &of,
        &target,
        r#"{"url": "https://example.com/img.jpg", "width": 800, "height": 600}"#,
        false,
    )));

    group.bench_function("video_content", |b| b.iter(|| from_str(
        &of,
        &target,
        r#"{"url": "https://example.com/video.mp4", "duration": 120}"#,
        false,
    )));

    group.finish();
}
