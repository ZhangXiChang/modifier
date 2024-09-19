use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{anyhow, Result};

pub trait MutexLock<T> {
    fn new_mutex(value: T) -> Self;
    fn lock(&self) -> Result<MutexGuard<T>>;
}
impl<T> MutexLock<T> for Mutex<T> {
    fn new_mutex(value: T) -> Self {
        Self::new(value)
    }
    fn lock(&self) -> Result<MutexGuard<T>> {
        self.lock().map_err(|err| anyhow!("{}", err))
    }
}
pub trait ReadWriteLock<T> {
    fn new_rwlock(value: T) -> Self;
    fn read(&self) -> Result<RwLockReadGuard<T>>;
    fn write(&self) -> Result<RwLockWriteGuard<T>>;
}
impl<T> ReadWriteLock<T> for RwLock<T> {
    fn new_rwlock(value: T) -> Self {
        Self::new(value)
    }
    fn read(&self) -> Result<RwLockReadGuard<T>> {
        self.read().map_err(|err| anyhow!("{}", err))
    }
    fn write(&self) -> Result<RwLockWriteGuard<T>> {
        self.write().map_err(|err| anyhow!("{}", err))
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
    fn lock(&self) -> Result<MutexGuard<T>> {
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
    fn read(&self) -> Result<RwLockReadGuard<T>> {
        self.value.read()
    }
    fn write(&self) -> Result<RwLockWriteGuard<T>> {
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
