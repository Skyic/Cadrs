use std::marker::PhantomData;

pub struct ParallelBatcher<T> {
    batch_size: usize,
    _phantom: PhantomData<T>,
}

impl<T: Clone> ParallelBatcher<T> {
    pub fn new(batch_size: usize) -> Self {
        Self { 
            batch_size,
            _phantom: PhantomData,
        }
    }

    pub fn process_batches<F, R>(&self, items: &[T], f: F) -> Vec<R>
    where F: Fn(&[T]) -> R, R: Clone {
        let mut results = Vec::new();
        
        for chunk in items.chunks(self.batch_size) {
            results.push(f(chunk));
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_batcher() {
        let batcher = ParallelBatcher::new(10);
        let items: Vec<i32> = (0..100).collect();
        
        let batches: Vec<Vec<i32>> = batcher.process_batches(&items, |batch| batch.to_vec());
        
        assert_eq!(batches.len(), 10);
        assert_eq!(batches[0].len(), 10);
    }
}
