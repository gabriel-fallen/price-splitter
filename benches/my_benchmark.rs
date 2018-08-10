#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use rand::prelude::*;

extern crate price_split;

use price_split::splitter::*;


fn bench_insert(c: &mut Criterion) {
  let nprices = 100;
  let mut rng = thread_rng();

  c.bench_function_over_inputs("insert", move |b, &&npairs| {
    let mut queue: Queue = vec![];
    for _ in 0 .. nprices {
      let price: Price = rng.gen();
      for _ in 0 .. npairs {
        let size: Size = rng.gen();
        insert(&mut queue, price, size, 1234);
      }
    }
    let p = rng.gen(); // FIXME: should I move it into `iter`?
    let s = rng.gen();
    b.iter(|| insert(&mut queue, p, s, 432))
  }, &[20, 70, 200]);
}

fn bench_split_half(c: &mut Criterion) {
  let mut queue: Queue = vec![];
  let mut total_size = 0;
  let mut rng = thread_rng();
  let nprices = rng.gen_range::<u32>(100, 1000);

  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    let npairs: u32  = rng.gen_range(10, 100);
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      total_size += size;
      insert(&mut queue, price, size, 1234);
    }
  }

  let s = total_size / 2;
  c.bench_function("split in halves", move |b| {
    let mut q = queue.clone();
    let     p = rng.gen();  // on average this should split prices roughly in equal halfs, like in qsort
    b.iter(|| split(&mut q, p, s))
  });
}

fn bench_split_n(c: &mut Criterion) {
  let mut rng = thread_rng();
  let nprices = 100;

  c.bench_function_over_inputs("split N", move |b, &&n| {
    let mut queue: Queue = vec![];
    let mut size_limit   = 0;
    let mut nelems       = 0;
    let p = std::i32::MAX;

    for _ in 0 .. nprices {
      let price: Price = rng.gen();
      let npairs: u32  = 70;
      for _ in 0 .. npairs {
        let size: Size = rng.gen();
        if nelems < n {
          size_limit += size;
          nelems += 1;
        }
        insert(&mut queue, price, size, 1234);
      }
    }

    b.iter(|| split(&mut queue, p, size_limit))
  }, &[1, 20, 100, 500]);
}

criterion_group!(benches, bench_insert, bench_split_half, bench_split_n);
criterion_main!(benches);