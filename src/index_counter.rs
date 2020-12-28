use std::marker::PhantomData;

use index_vec::Idx;

#[derive(Debug)]
pub struct IndexCounter<I: Idx> {
    next_id: I,
    _phantom: PhantomData<I>,
}

impl<I: Idx> IndexCounter<I> {
    pub fn new() -> Self {
        IndexCounter { next_id: I::from_usize(0), _phantom: PhantomData }
    }

    pub fn next(&mut self) -> I {
        let next = self.next_id;
        self.next_id = I::from_usize(self.next_id.index() + 1);
        next
    }

    pub fn len(&self) -> usize { self.next_id.index() }
}

impl<I: Idx> Default for IndexCounter<I> {
    fn default() -> Self {
        IndexCounter::new()
    }
}