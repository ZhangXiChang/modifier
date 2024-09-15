use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub trait MutexLock<T> {
    fn new_mutex(value: T) -> Self;
    fn lock(&self) -> MutexGuard<T>;
}
impl<T> MutexLock<T> for Mutex<T> {
    fn new_mutex(value: T) -> Self {
        Self::new(value)
    }
    fn lock(&self) -> MutexGuard<T> {
        self.lock().unwrap()
    }
}
pub trait ReadWriteLock<T> {
    fn new_rwlock(value: T) -> Self;
    fn read(&self) -> RwLockReadGuard<T>;
    fn write(&self) -> RwLockWriteGuard<T>;
}
impl<T> ReadWriteLock<T> for RwLock<T> {
    fn new_rwlock(value: T) -> Self {
        Self::new(value)
    }
    fn read(&self) -> RwLockReadGuard<T> {
        self.read().unwrap()
    }
    fn write(&self) -> RwLockWriteGuard<T> {
        self.write().unwrap()
    }
}

#[derive(Default)]
pub struct Pointer<L> {
    value: Arc<L>,
}
impl<L, T> MutexLock<T> for Pointer<L>
where
    L: MutexLock<T>,
{
    fn new_mutex(value: T) -> Self {
        Self {
            value: Arc::new(L::new_mutex(value)),
        }
    }
    fn lock(&self) -> MutexGuard<T> {
        self.value.lock()
    }
}
impl<L, T> ReadWriteLock<T> for Pointer<L>
where
    L: ReadWriteLock<T>,
{
    fn new_rwlock(value: T) -> Self {
        Self {
            value: Arc::new(L::new_rwlock(value)),
        }
    }
    fn read(&self) -> RwLockReadGuard<T> {
        self.value.read()
    }
    fn write(&self) -> RwLockWriteGuard<T> {
        self.value.write()
    }
}
impl<L> Clone for Pointer<L> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
unsafe impl<L> Send for Pointer<L> {}
unsafe impl<L> Sync for Pointer<L> {}
