#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    fmt::Debug,
    hash::{
        BuildHasherDefault,
        Hasher,
    },
    marker::PhantomData,
};
#[cfg(feature = "std")]
use std::collections::{
    HashMap,
    HashSet,
};

#[cfg(feature = "std")]
pub type NoHashMap<K, V> = HashMap<K, V, StableState<K>>;

#[cfg(feature = "std")]
pub type NoHashSet<T> = HashSet<T, StableState<T>>;

pub type StableState<T> = BuildHasherDefault<NoHashHasher<T>>;

trait NoHashable {}

macro_rules! impl_nohashable {
    ($($ty:ty),+) => {
        $(impl NoHashable for $ty {})+
    };
}
impl_nohashable!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, char);

#[derive(Clone, Copy, Default)]
pub struct NoHashHasher<T> {
    value: u64,
    _t:    PhantomData<T>,

    #[cfg(debug_assertions)]
    written: bool,
}

impl<T: NoHashable> Debug for NoHashHasher<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NoHashHasher").field("value", &self.value).finish()
    }
}

macro_rules! write_impl {
    ($name:ident, $ty:ty) => {
        #[inline]
        fn $name(&mut self, i: $ty) {
            #[cfg(debug_assertions)]
            {
                if self.written {
                    panic!("NoHashHasher: `write` called more than once on the same hasher");
                }
                self.written = true;
            }
            self.value = i as _;
        }
    };
}

#[rustfmt::skip]
impl<T: NoHashable> Hasher for NoHashHasher<T> {
    fn finish(&self) -> u64 {
        self.value as _
    }

    fn write(&mut self, _: &[u8]) {
        panic!("NoHashHasher is intended for word-sized keys only");
    }

    write_impl!(write_u8, u8);
    write_impl!(write_u16, u16);
    write_impl!(write_u32, u32);
    write_impl!(write_u64, u64);
    write_impl!(write_usize, usize);
    write_impl!(write_i8, i8);
    write_impl!(write_i16, i16);
    write_impl!(write_i32, i32);
    write_impl!(write_i64, i64);
    write_impl!(write_isize, isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher_direct_usage_unsigned() {
        let mut hasher = NoHashHasher::<u32>::default();
        hasher.write_u32(12345);
        assert_eq!(hasher.finish(), 12345);
    }

    #[test]
    fn hasher_direct_usage_signed() {
        let mut hasher = NoHashHasher::<i64>::default();
        hasher.write_i64(-1);
        assert_eq!(hasher.finish(), -1i64 as u64);
    }

    #[test]
    #[cfg(feature = "std")]
    fn nohashmap_works() {
        let mut map = NoHashMap::<u64, &str>::default();
        map.insert(42, "hello");
        map.insert(101, "world");

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&42), Some(&"hello"));
        assert_eq!(map.get(&101), Some(&"world"));
        assert_eq!(map.get(&999), None);
        assert!(map.contains_key(&42));
    }

    #[test]
    #[cfg(feature = "std")]
    fn nohashset_works() {
        let mut set = NoHashSet::<i32>::default();
        set.insert(-10);
        set.insert(0);
        set.insert(500);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&-10));
        assert!(set.contains(&500));
        assert!(!set.contains(&1));
    }

    #[test]
    #[should_panic(expected = "NoHashHasher is intended for word-sized keys only")]
    fn write_slice_panics() {
        let mut hasher = NoHashHasher::<u8>::default();
        hasher.write(b"this is not an integer"); // should panic
    }
}
