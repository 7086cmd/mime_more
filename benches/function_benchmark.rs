#![allow(dead_code)]

use std::str::FromStr;
use criterion::{criterion_group, Criterion, criterion_main};
use mime_more::Mime;

fn bench_from_str(c: &mut Criterion) {
    c.bench_function("ModuleType::from_known_str", |b| {
        b.iter(|| {
            use std::str::FromStr;
            Mime::from_str("application/json").unwrap();
        })
    });
}

#[cfg(feature = "extension")]
fn bench_from_ext(c: &mut Criterion) {
    c.bench_function("ModuleType::from_ext", |b| {
        b.iter(|| {
            Mime::from_ext("json").unwrap();
        })
    });
}

#[cfg(feature = "extension-light")]
fn bench_from_ext_light(c: &mut Criterion) {
    c.bench_function("ModuleType::from_ext_light", |b| {
        b.iter(|| {
            Mime::from_ext_light("json").unwrap();
        })
    });
}

#[cfg(feature = "magic")]
fn bench_from_content(c: &mut Criterion) {
    c.bench_function("ModuleType::from_content", |b| {
        b.iter(|| {
            let data = &[
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
            ];

            Mime::from_content(data).unwrap();
        })
    });
}

#[cfg(feature = "texture")]
fn bench_is_texture(c: &mut Criterion) {
    c.bench_function("ModuleType::is_texture_std", |b| {
        b.iter(|| {
            let data = &[
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
            ];

            mime_more::texture::is_texture_std(data);
        })
    });

    c.bench_function("ModuleType::is_texture_mime", |b| {
        b.iter(|| {
            let mime = Mime::from_str("image/png").unwrap();

            mime_more::texture::is_texture_mime(&mime.mime);
        })
    });

    c.bench_function("ModuleType::is_texture_manual", |b| {
        b.iter(|| {
            let data = &[
                0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
            ];

            mime_more::texture::is_texture_manual(data, 512);
        })
    });
}

#[cfg(feature = "texture")]
fn bench_isnt_texture(c: &mut Criterion) {
    c.bench_function("ModuleType::is_texture_std", |b| {
        b.iter(|| {
            let data = b"Hello, World!";

            mime_more::texture::is_texture_std(data);
        })
    });

    c.bench_function("ModuleType::is_texture_mime", |b| {
        b.iter(|| {
            let mime = Mime::from_str("text/plain").unwrap();

            mime_more::texture::is_texture_mime(&mime.mime);
        })
    });

    c.bench_function("ModuleType::is_texture_manual", |b| {
        b.iter(|| {
            let data = b"Hello, World!";

            mime_more::texture::is_texture_manual(data, 512);
        })
    });
}

#[cfg(feature = "texture")]
fn bench_large_file_isnt_texture(c: &mut Criterion) {
    c.bench_function("ModuleType::is_texture_std", |b| {
        b.iter(|| {
            let data = vec![0; 1024 * 1024];

            mime_more::texture::is_texture_std(&data);
        })
    });

    c.bench_function("ModuleType::is_texture_mime", |b| {
        b.iter(|| {
            let mime = Mime::from_str("text/plain").unwrap();

            mime_more::texture::is_texture_mime(&mime.mime);
        })
    });

    c.bench_function("ModuleType::is_texture_manual", |b| {
        b.iter(|| {
            let data = vec![0; 1024 * 1024];

            mime_more::texture::is_texture_manual(&data, 512);
        })
    });
}

#[cfg(feature = "dataurl")]
fn bench_dataurl(c: &mut Criterion) {
    c.bench_function("Dataurl::from_path", |b| {
        b.iter(|| {
            let path = std::path::Path::new("Cargo.toml");

            mime_more::Dataurl::from_path(path).unwrap();
        })
    });
}

#[cfg(feature = "dataurl")]
fn decode_texture(c: &mut Criterion) {
    c.bench_function("Dataurl::from_str", |b| {
        b.iter(|| {
            let dataurl = mime_more::dataurl::Dataurl::from_str("data:image/png;base64,iVBORw0KGgoRRRQZGYEA").unwrap();
            assert_eq!(dataurl.mime, "image/png");
        })
    });
}

criterion_group!(
    benches, bench_from_str, bench_from_ext, bench_from_ext_light,
    bench_from_content, bench_is_texture, bench_isnt_texture,
    bench_large_file_isnt_texture, bench_dataurl, decode_texture
);

criterion_main!(benches);
