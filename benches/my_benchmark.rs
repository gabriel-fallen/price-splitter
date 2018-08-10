#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use rand::prelude::*;
use std::cell::RefCell;

extern crate price_split;

use price_split::splitter::*;


fn bench_insert_2000(c: &mut Criterion) {
  let nprices = 100;
  let npairs  = 20;
  let mut rng = thread_rng();

  let queue = RefCell::new(Queue::new(vec![]));
  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      queue.borrow_mut().insert(price, size, 1234);
    }
  }

  c.bench_function("insert 2000", move |b| {
    b.iter_with_setup(|| {
      queue.borrow_mut().pop_rand();
      (rng.gen(), rng.gen())
    }, |(p, s)| queue.borrow_mut().insert(p, s, 432))
  });
}

fn bench_insert_7000(c: &mut Criterion) {
  let nprices = 100;
  let npairs  = 70;
  let mut rng = thread_rng();

  let queue = RefCell::new(Queue::new(vec![]));
  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      queue.borrow_mut().insert(price, size, 1234);
    }
  }

  c.bench_function("insert 7000", move |b| {
    b.iter_with_setup(|| {
      queue.borrow_mut().pop_rand();
      (rng.gen(), rng.gen())
    }, |(p, s)| queue.borrow_mut().insert(p, s, 432))
  });
}

fn bench_insert_20000(c: &mut Criterion) {
  let nprices = 100;
  let npairs  = 200;
  let mut rng = thread_rng();

  let queue = RefCell::new(Queue::new(vec![]));
  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      queue.borrow_mut().insert(price, size, 1234);
    }
  }

  c.bench_function("insert 20000", move |b| {
    b.iter_with_setup(|| {
      queue.borrow_mut().pop_rand();
      (rng.gen(), rng.gen())
    }, |(p, s)| queue.borrow_mut().insert(p, s, 432))
  });
}

fn bench_split_half(c: &mut Criterion) {
  let mut queue      = Queue::new(vec![]);
  let mut total_size = 0;
  let mut rng        = thread_rng();
  let nprices: u32   = rng.gen_range(100, 1000);

  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    let npairs: u32  = rng.gen_range(10, 100);
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      total_size += size;
      queue.insert(price, size, 1234);
    }
  }

  let s = total_size / 2;
  c.bench_function("split in halves", move |b| {
    b.iter_with_setup(|| {
      let q = queue.clone();
      let p = rng.gen();  // on average this should split prices roughly in equal halfs, like in qsort
      (q, p)
    }, |(mut q, p)| drop(q.split(p, s)))
  });
}

fn bench_split_n(c: &mut Criterion) {
  let mut rng     = thread_rng();
  let     nprices = 100;
  let     npairs  = 70;
  let     queue   = RefCell::new(Queue::new(vec![]));
  let mut nelems  = 0;

  for _ in 0 .. nprices {
    let price: Price = rng.gen();
    for _ in 0 .. npairs {
      let size: Size = rng.gen();
      queue.borrow_mut().insert(price, size, 1234);
    }
  }

  c.bench_function_over_inputs("split N", move |b, &&n| {
    b.iter_with_setup(|| {
      let mut size_limit = 0;

      // restore original number of elements
      for _ in 0 .. nelems {
        let price: Price = rng.gen();
        let size: Size = rng.gen();
        queue.borrow_mut().insert(price, size, 1234);
      }

      nelems = 0;
      let q = queue.borrow();
      for i in 0 .. q.len() {
        for j in 0 .. q.get_len(i) {
          let size = q.get_size(i, j);
          if nelems < n {
            size_limit += size;
            nelems += 1;
          }
        }
      }

      size_limit
    }, |size_limit| {
      let mut q = queue.borrow_mut();
      let     r = q.split(std::i32::MAX, size_limit);
      *q = r;
    })
  }, &[1, 20, 100, 500]);
}

criterion_group!(benches, bench_insert_2000, bench_insert_7000, bench_insert_20000, bench_split_half, bench_split_n);
criterion_main!(benches);