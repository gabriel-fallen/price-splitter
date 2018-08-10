extern crate rand;

pub mod splitter {
  pub type Price = i32;
  pub type Size  = u32;
  pub type Meta  = u128;

  #[derive(Clone, Debug, PartialEq)]
  pub struct Queue(Vec<(Price, Vec<(Size, Meta)>)>);

  impl Queue {
    pub fn new(q: Vec<(Price, Vec<(Size, Meta)>)>) -> Queue {
      Queue(q)
    }

    pub fn len(&self) -> usize {
      self.0.len()
    }

    pub fn get_len(&self, i: usize) -> usize {
      self.0[i].1.len()
    }

    pub fn get_size(&self, i: usize, j: usize) -> Size {
      self.0[i].1[j].0
    }

    pub fn pop_rand(&mut self) {
      use rand::prelude::*;
      let mut rng = thread_rng();
      let ind: usize = rng.gen_range(0, self.0.len());
      self.0[ind].1.pop();
    }

    pub fn append(&mut self, rest: &mut Queue) {
      self.0.append(&mut rest.0);
    }
  }

  pub trait Split {
    fn insert(&mut self, p: Price, s: Size, m: Meta);
    fn split(&mut self, p: Price, s: Size) -> Self;
  }

  impl Split for Queue {

    /// Inserts given `Price`, `Size` and `Meta` into given `Queue` __in place__.
    /// The `Queue` should be sorted by `Price` and stays sorted.
    /// Worst-case `O(q.len())` because we shift elements when inserting.
    /// Best-case `O(log(q.len()))`.
    fn insert(&mut self, p: Price, s: Size, m: Meta) {
      match self.0.binary_search_by_key(&p, |&(price, _)| price) {
        Ok(ind) => {
          // p is already there, just add another (s, m) to it
          self.0[ind].1.push((s, m));
        }
        Err(ind) => {
          // no such price was there, insert a fresh one in an `ind` position
          self.0.insert(ind, (p, vec![(s, m)]));
        }
      }
    }

    /// Splits given `Queue` according to given `Price` and `Size`.
    /// `split` __does__ mutate `q`.
    /// All the elements left in `q` are `<= p` and sum of all `Size`s is `<= s`.
    /// All other elements are returned as a result.
    /// A `(Price, Vec<(Size, Meta)>)` pair might be split between `q` and result.
    /// Complexity is `O(q.len()*q.iter().map(|(_, v)| v.len()).max())`
    /// if complexity of `split_off` is no more than `O(q.len())`.
    fn split(&mut self, p: Price, s: Size) -> Queue {
      let mut size_acc = 0;

      let mindex = self.0.iter().position(|&(price, ref v)| {
        size_acc += v.iter().map(|&(size, _)| size).sum::<u32>();
        price > p || size_acc > s
      });

      if let Some(ind) = mindex {
        // everything < ind is guaranteed to be under the limits
        let mut res = self.0.split_off(ind);

        // now what about res[0]? Should we split it?
        let price = res[0].0;
        if price <= p {
          // we can split
          size_acc -= res[0].1.iter().map(|&(size, _)| size).sum::<u32>();
          let (addition, rest) = split_size(&res[0].1, size_acc, s);

          self.0.push((price, addition));
          res[0].1 = rest;
        }

        Queue(res)
      } else {
        // all elements are under the limits
        // keep original vector as it is, return empty one
        Queue(vec![])
      }
    }
  }

  fn split_size(v: &Vec<(Size, Meta)>, mut acc: u32, lim: u32) -> (Vec<(Size, Meta)>, Vec<(Size, Meta)>) {
    let mut addition: Vec<(Size, Meta)> = vec![];
    let mut rest: Vec<(Size, Meta)>     = vec![];
    let mut iter = v.iter();

    loop {
      if let Some(&(s, m)) = iter.next() {
        if acc + s <= lim {
          // add it altogeather
          addition.push((s, m));
          acc += s
        } else {
          // split
          let cap = lim - acc;
          let rem = s - cap;
          // there was no specification how to split Meta, so I just copy it
          addition.push((cap, m));
          rest.push((rem, m));
          break; // we're done here
        }
      } else {
        break; // should be unreachable, actually...
      }
    }

    // add to `rest` whatever left in the iterator
    iter.for_each(|&elem| rest.push(elem));

    (addition, rest)
  }
}

#[cfg(test)]
mod tests {
  use splitter::*;

  #[test]
  fn split_3_15() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(3, 15);
    assert_eq!(rest, Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]));
    assert_eq!(q, Queue::new(vec![]));
  }

  #[test]
  fn split_6_15() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(6, 15);
    assert_eq!(rest, Queue::new(vec![(5, vec![(15, 3)]), (7, vec![(10, 40), (20, 50)])]));
    assert_eq!(q, Queue::new(vec![(5, vec![(10, 2), (5, 3)])]));
  }

  #[test]
  fn split_8_15() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(8, 15);
    assert_eq!(rest, Queue::new(vec![(5, vec![(15, 3)]), (7, vec![(10, 40), (20, 50)])]));
    assert_eq!(q, Queue::new(vec![(5, vec![(10, 2), (5, 3)])]));
  }

  #[test]
  fn split_8_35() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(8, 35);
    assert_eq!(rest, Queue::new(vec![(7, vec![(5, 40), (20, 50)])]));
    assert_eq!(q, Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(5, 40)])]));
  }

  #[test]
  fn split_6_100() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(6, 100);
    assert_eq!(rest, Queue::new(vec![(7, vec![(10, 40), (20, 50)])]));
    assert_eq!(q, Queue::new(vec![(5, vec![(10, 2), (20, 3)])]));
  }

  #[test]
  fn split_100_100() {
    let mut q = Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]);
    let rest  = q.split(100, 100);
    assert_eq!(rest, Queue::new(vec![]));
    assert_eq!(q, Queue::new(vec![(5, vec![(10, 2), (20, 3)]), (7, vec![(10, 40), (20, 50)])]));
  }

  #[test]
  fn insert_empty() {
    let mut queue = Queue::new(vec![]);
    queue.insert(10, 2, 0);
    assert_eq!(queue, Queue::new(vec![(10, vec![(2, 0)])]));
  }

  #[test]
  fn insert_new() {
    let mut queue = Queue::new(vec![(10, vec![(2, 0)])]);
    queue.insert(20, 3, 20);
    assert_eq!(queue, Queue::new(vec![(10, vec![(2, 0)]), (20, vec![(3, 20)])]));
  }

  #[test]
  fn insert_existing() {
    let mut queue = Queue::new(vec![(10, vec![(2, 0)])]);
    queue.insert(10, 3, 20);
    assert_eq!(queue, Queue::new(vec![(10, vec![(2, 0), (3, 20)])]));
  }
}
