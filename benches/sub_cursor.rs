// https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slice::IoSlice;
use sub_cursor::SubCursor;

fn bench_seek_start(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::from(Cursor::new(vec![0; 200]))
        .start(20)
        .end(200)
        .preserve(false);

    c.bench_function("seek_start", |b| {
        b.iter(|| {
            sub_cursor.seek(black_box(SeekFrom::Start(200))).unwrap();
        })
    });

    let mut io_slice = IoSlice::new(Cursor::new(vec![0; 200]), 20, 200).unwrap();

    c.bench_function("IoSlice: seek_start", |b| {
        b.iter(|| {
            io_slice.seek(black_box(SeekFrom::Start(200))).unwrap();
        })
    });
}

fn bench_seek_current(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::from(Cursor::new(vec![0; 200]))
        .start(20)
        .end(200)
        .preserve(false);

    c.bench_function("seek_current", |b| {
        b.iter(|| {
            sub_cursor.seek(black_box(SeekFrom::Current(200))).unwrap();
            sub_cursor.seek(black_box(SeekFrom::Start(0))).unwrap();
        })
    });

    let mut io_slice = IoSlice::new(Cursor::new(vec![0; 200]), 20, 200).unwrap();

    c.bench_function("IoSlice: seek_current", |b| {
        b.iter(|| {
            io_slice.seek(black_box(SeekFrom::Current(200))).unwrap();
            io_slice.seek(black_box(SeekFrom::Start(0))).unwrap();
        })
    });
}

fn bench_seek_end(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::from(Cursor::new(vec![0; 200]))
        .start(20)
        .end(200)
        .preserve(false);

    c.bench_function("seek_end", |b| {
        b.iter(|| {
            sub_cursor.seek(black_box(SeekFrom::End(200))).unwrap();
        })
    });
}

/*
#[bench]
fn bench_seek_start(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    b.iter(|| {
        sub_cursor.seek(SeekFrom::Start(200)).unwrap();
    });
}

#[bench]
fn bench_seek_current(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    b.iter(|| {
        sub_cursor.seek(SeekFrom::Current(200)).unwrap();
    });
}

#[bench]
fn bench_seek_end(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    b.iter(|| {
        sub_cursor.seek(SeekFrom::End(200)).unwrap();
    });
}

#[bench]
fn bench_read(c: &mut Criterion) {
    let buffer = (0..255).into_iter().map(|x| x as u8).collect::<Vec<_>>();
    let mut sub_cursor = SubCursor::from(Cursor::new(buffer))
        .start(20)
        .end(100)
        .preserve(false);

    let mut result_buffer = [0; 100];

    b.iter(|| {
        sub_cursor.read(&mut result_buffer).unwrap();
    });
}

#[bench]
fn bench_preserve(c: &mut Criterion) {
    let buffer = (0..255).into_iter().map(|x| x as u8).collect::<Vec<_>>();

    let cursor = Arc::new(Mutex::new(Cursor::new(buffer)));

    let mut sub_cursor = SubCursor::from(cursor.clone())
        .start(20)
        .end(100)
        .preserve(true);

    let mut result_buffer = [0; 100];

    cursor.lock().unwrap().set_position(20); // <- the position shouldn't be modified by the SubCursor

    // we can simply do some normal reads?
    b.iter(|| {
        sub_cursor.read(&mut result_buffer).unwrap();
    });
}

#[bench]
fn bench_write(c: &mut Criterion) {
    let mut sub_cursor = SubCursor::new().start(20).end(100).preserve(false);

    b.iter(|| {
        sub_cursor.write(&[0, 1, 2, 3, 4]).unwrap();
    });
}*/

criterion_group!(
    benches,
    bench_seek_start,
    bench_seek_current,
    bench_seek_end
);
criterion_main!(benches);
