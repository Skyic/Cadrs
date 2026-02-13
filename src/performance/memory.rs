pub struct SimpleArena {
    buffer: Vec<u8>,
    position: usize,
}

impl SimpleArena {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0u8; capacity],
            position: 0,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.position = 0;
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn used(&self) -> usize {
        self.position
    }

    #[inline]
    pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.position + size <= self.buffer.len() {
            let ptr = &mut self.buffer[self.position..self.position + size];
            self.position += size;
            Some(ptr)
        } else {
            None
        }
    }
}

pub struct ObjectPool<T> {
    pool: Vec<T>,
    max_size: usize,
}

impl<T: Default> ObjectPool<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::with_capacity(max_size),
            max_size,
        }
    }

    #[inline]
    pub fn get(&mut self) -> T {
        self.pool.pop().unwrap_or_default()
    }

    #[inline]
    pub fn release(&mut self, obj: T) {
        if self.pool.len() < self.max_size {
            self.pool.push(obj);
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena() {
        let mut arena = SimpleArena::new(1024);
        let chunk = arena.allocate(100);
        assert!(chunk.is_some());
        assert_eq!(arena.used(), 100);
    }

    #[test]
    fn test_object_pool() {
        let mut pool: ObjectPool<i32> = ObjectPool::new(10);
        let val = pool.get();
        assert_eq!(val, 0);
        
        pool.release(42);
        let val = pool.get();
        assert_eq!(val, 42);
    }
}
