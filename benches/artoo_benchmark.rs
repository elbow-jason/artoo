use artoo::Tree as Art;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use skiplist::SkipMap;
use std::collections::BTreeMap as BTree;
use std::collections::HashMap;

fn art_new(n: usize) -> Art<usize> {
    let mut tree = Art::<usize>::new();
    for i in 0..n {
        let arr = i.to_be_bytes();
        let key = &arr[..];
        let _ins = tree.insert(key, i);
        // assert_eq!(ins, None);
        // assert_eq!(tree.get(key), Some(&i));
    }
    tree
}

fn btree_new(n: usize) -> BTree<[u8; 8], usize> {
    let mut tree = BTree::<[u8; 8], usize>::new();
    for i in 0..n {
        let arr = i.to_be_bytes();
        let _ins = tree.insert(arr.clone(), i);
        // assert_eq!(ins, None);
        // assert_eq!(tree.get(&arr[..]), Some(&i));
    }
    tree
}

fn hashmap_new(n: usize) -> HashMap<[u8; 8], usize> {
    let mut hm = HashMap::<[u8; 8], usize>::new();
    for i in 0..n {
        let arr = i.to_be_bytes();
        let _ins = hm.insert(arr.clone(), i);
        // assert_eq!(ins, None);
        // assert_eq!(hm.get(&arr[..]), Some(&i));
    }
    hm
}

fn skip_map_new(n: usize) -> SkipMap<[u8; 8], usize> {
    let mut sm = SkipMap::<[u8; 8], usize>::new();
    for i in 0..n {
        let arr = i.to_be_bytes();
        let _ins = sm.insert(arr.clone(), i);
        // assert_eq!(ins, None);
        // assert_eq!(hm.get(&arr[..]), Some(&i));
    }
    sm
}

// fn bench_new(c: &mut Criterion) {
//     c.bench_function("art_new", |b| b.iter(|| _ = Art::<i32>::new()));
//     c.bench_function("btree_new", |b| b.iter(|| _ = BTree::<&[u8], i32>::new()));
//     c.bench_function("hashmap_new", |b| {
//         b.iter(|| _ = HashMap::<&[u8], i32>::new())
//     });
//     c.bench_function("skipmap_new", |b| {
//         b.iter(|| _ = SkipMap::<&[u8], i32>::new())
//     });
// }

fn bench_insert_100k(c: &mut Criterion) {
    let start_n = 100_000;
    let mut group = c.benchmark_group("insert_100k");
    group.throughput(Throughput::Elements(1));
    group.sample_size(100);
    group.bench_function("art", |b| {
        let mut art = art_new(start_n);
        let mut i: usize = start_n;
        b.iter(|| {
            art.insert(&i.to_be_bytes()[..], i);
            i += 1;
        });
    });
    group.bench_function("btree", |b| {
        let mut btree = btree_new(start_n);
        let mut i: usize = start_n;
        b.iter(|| {
            btree.insert(i.to_be_bytes(), i);
            i += 1;
        });
    });

    group.bench_function("hash_map", |b| {
        let mut hm = hashmap_new(start_n);
        let mut i: usize = start_n;
        b.iter(|| {
            hm.insert(i.to_be_bytes(), i);
            i += 1;
        });
    });

    group.bench_function("skip_map", |b| {
        let mut sm = skip_map_new(start_n);
        let mut i: usize = start_n;
        b.iter(|| {
            sm.insert(i.to_be_bytes(), i);
            i += 1;
        });
    });
}

fn bench_get_100k(c: &mut Criterion) {
    let n = 100_000;
    let mut group = c.benchmark_group("get_100k");
    group.throughput(Throughput::Elements(1));
    group.sample_size(100);
    group.bench_function("art", |b| {
        let art = art_new(n);
        let mut i: usize = 0;
        b.iter(|| {
            art.get(&i.to_be_bytes());
            i += 1;
        });
    });

    group.bench_function("btree", |b| {
        let btree = btree_new(n);
        let mut i: usize = 0;
        b.iter(|| {
            btree.get(&i.to_be_bytes());
            i += 1;
        });
    });

    group.bench_function("hash_map", |b| {
        let hm = hashmap_new(n);
        let mut i: usize = 0;
        b.iter(|| {
            hm.get(&i.to_be_bytes());
            i += 1;
        });
    });

    group.bench_function("skip_map", |b| {
        let sm = skip_map_new(n);
        let mut i: usize = 0;
        b.iter(|| {
            sm.get(&i.to_be_bytes());
            i += 1;
        });
    });
}

criterion_group!(
    benches,
    // bench_new,
    bench_insert_100k,
    bench_get_100k,
    // bench_get_10m,
);
criterion_main!(benches);
