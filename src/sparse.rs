use std::ops::{Deref, DerefMut};

type INT = u16;

const NONE_MARKER: INT = INT::MAX;

#[derive(Clone, Copy, Debug, Default)]
struct DenseIndex(INT);

impl DenseIndex {
    const fn none() -> Self {
        Self(NONE_MARKER)
    }
    const fn index(self) -> usize {
        self.0 as usize
    }
    const fn is_none(self) -> bool {
        self.0 == NONE_MARKER
    }
    const fn is_some(self) -> bool {
        !self.is_none()
    }
}

const fn to_index(num: SparseIndex) -> usize {
    num as usize
}

type SparseIndex = INT;

#[derive(Debug, Default)]
pub struct SparseVec<T> {
    dense: Dense<T>,
    sparse: Vec<DenseIndex>,
}

impl<T> SparseVec<T> {
    /// Creates a new sparse vector with a maximum size of `max_size`
    pub fn new(max_size: INT) -> Self {
        SparseVec {
            dense: Dense::new(),
            sparse: vec![DenseIndex::none(); max_size as usize],
        }
    }

    pub fn entry(&mut self, index: INT) -> Entry<'_, T> {
        let sparse_index = index;
        if let Some(dense_index) = self.contains_dense(sparse_index) {
            Entry::Occupied(OccupiedEntry {
                sparse_index,
                dense_index,
                inner: self,
            })
        }
        else {
            Entry::Vacant(VacantEntry {
                sparse_index,
                inner: self,
            })
        }
    }

    fn get_dense_index(&self, sparse_index: SparseIndex) -> Option<DenseIndex> {
        self.sparse.get(to_index(sparse_index)).copied()
    }

    fn set_dense_index(&mut self, sparse_index: SparseIndex, dense_index: DenseIndex) {
        set_dense_index(&mut self.sparse, sparse_index, dense_index)
    }

    fn set_dense_index_none(&mut self, sparse_index: SparseIndex) {
        set_dense_index(&mut self.sparse, sparse_index, DenseIndex::none());
    }

    fn contains_dense(&self, sparse_index: SparseIndex) -> Option<DenseIndex> {
        let dense_index = self.get_dense_index(sparse_index);
        dense_index.filter(|x| x.is_some())
    }

    pub fn contains(&self, index: INT) -> bool {
        let sparse_index = index;
        self.contains_dense(sparse_index).is_some()
    }

    pub fn insert(&mut self, index: INT, value: T) -> Option<T> {
        self.entry(index)
            .insert(value)
    }

    pub fn get(&self, index: INT) -> Option<&T> {
        let sparse_index = index;
        let dense_index = self.get_dense_index(sparse_index)?;

        if dense_index.is_some() {
            Some(self.dense.get(dense_index))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: INT) -> Option<&mut T> {
        match self.entry(index) {
            Entry::Occupied(entry) => Some(entry.into_mut()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn remove(&mut self, index: INT) -> Option<T> {
        match self.entry(index) {
            Entry::Occupied(entry) => Some(entry.remove()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn sort(&mut self) {
        self.dense.sort();

        // this has to be doen to work around lifetime issues
        let dense = &mut self.dense;
        let dense_indexes = &mut self.sparse;

        // elements in dense were moved around so dense_indexes are pointing to the wrong place
        // O(n) (where n is the size of dense) operation to fix dense_indexes
        dense.items.iter().enumerate().for_each(|(i, x)| {
            let sparse_index = x.1;
            let dense_index = DenseIndex(i as INT);
            set_dense_index(dense_indexes, sparse_index, dense_index);
        });
    }

    // when a dense item is moved around, it will still point to the correct sparse index
    // but the dense index will be pointing to a different, possibly invalid dense item
    // the index to reset is obtained from the dense array
    fn reset_sparse_mapping(&mut self, dense_index: DenseIndex) {
        if let Some(sparse_index) = self.dense.get_sparse_index(dense_index) {
            self.set_dense_index(sparse_index, dense_index);
        }
    }

    pub fn clear(&mut self) {
        let sparse = &mut self.sparse;
        let dense = &mut self.dense;
        for (_, index) in dense.items.drain(..) {
            let sparse_index = to_index(index);
            sparse[sparse_index] = DenseIndex::none();
        }
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }
}

impl<T> Deref for SparseVec<T> {
    type Target = [Item<T>];

    fn deref(&self) -> &Self::Target {
        &self.dense.items
    }
}

impl<T> DerefMut for SparseVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dense.items
    }
}

// goofy code deduplication for SparseVec::sort
fn set_dense_index(
    dense_indexes: &mut Vec<DenseIndex>,
    sparse_index: SparseIndex,
    dense_index: DenseIndex,
) {
    dense_indexes
        .get_mut(to_index(sparse_index))
        .map(|x| *x = dense_index);
}

// simulating HashMap entry api
pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

impl<'a, T> Entry<'a, T> {
    pub fn index(&self) -> INT {
        match self {
            Entry::Occupied(entry) => entry.index(),
            Entry::Vacant(entry) => entry.index(),
        }
    }

    pub fn insert(self, value: T) -> Option<T> {
        match self {
            Entry::Occupied(mut entry) => Some(entry.insert(value)),
            Entry::Vacant(entry) => {
                entry.insert(value);
                None
            }
        }
    }

    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    pub fn or_insert_with_index<F: FnOnce(INT) -> T>(self, default: F) -> &'a T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.sparse_index);
                entry.insert(value)
            }
        }
    }

    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(x) => Entry::Vacant(x),
        }
    }

    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }
}

impl<'a, T: Default> Entry<'a, T> {
    pub fn or_default(self) -> &'a mut T {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(T::default()),
        }
    }
}

pub struct OccupiedEntry<'a, T> {
    sparse_index: SparseIndex,
    dense_index: DenseIndex,
    inner: &'a mut SparseVec<T>,
}

impl<'a, T> OccupiedEntry<'a, T> {
    pub fn index(&self) -> INT {
        self.sparse_index
    }

    pub fn get(&self) -> &T {
        self.inner.dense.get(self.dense_index)
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.dense.get_mut(self.dense_index)
    }

    pub fn into_mut(self) -> &'a mut T {
        self.inner.dense.get_mut(self.dense_index)
    }

    pub fn insert(&mut self, value: T) -> T {
        self.inner.dense.replace(self.dense_index, value)
    }

    pub fn remove(self) -> T {
        let item = self.inner.dense.swap_remove(self.dense_index);
        self.inner.set_dense_index_none(self.sparse_index);
        self.inner.reset_sparse_mapping(self.dense_index);
        item
    }
}

pub struct VacantEntry<'a, T> {
    sparse_index: SparseIndex,
    inner: &'a mut SparseVec<T>,
}

impl<'a, T> VacantEntry<'a, T> {
    pub fn index(&self) -> INT {
        self.sparse_index
    }

    pub fn insert(self, value: T) -> &'a mut T {
        let (dense_index, item) = self.inner.dense.push_mut(self.sparse_index, value);
        self.inner.sparse[to_index(self.sparse_index)] = dense_index;
        item
    }

    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        let (dense_index, item) = self.inner.dense.push_mut(self.sparse_index, value);
        self.inner.sparse[to_index(self.sparse_index)] = dense_index;
        OccupiedEntry {
            sparse_index: self.sparse_index,
            dense_index,
            inner: self.inner,
        }
    }
}

#[derive(Debug, Default)]
struct Dense<T> {
    items: Vec<Item<T>>,
}

impl<T> Dense<T> {
    fn new() -> Self {
        Dense { items: Vec::new() }
    }

    fn push(&mut self, sparse_index: SparseIndex, value: T) -> DenseIndex {
        self.push_mut(sparse_index, value).0
    }

    fn push_mut(&mut self, sparse_index: SparseIndex, value: T) -> (DenseIndex, &mut T) {
        let dense_index = self.items.len();
        self.items.push((value, sparse_index));
        (
            DenseIndex(dense_index as INT),
            &mut self.items[dense_index].0,
        )
    }

    fn get(&self, dense_index: DenseIndex) -> &T {
        &self.items[dense_index.index()].0
    }

    fn get_mut(&mut self, dense_index: DenseIndex) -> &mut T {
        &mut self.items[dense_index.index()].0
    }

    fn get_sparse_index(&self, dense_index: DenseIndex) -> Option<SparseIndex> {
        self.items.get(dense_index.index()).map(|x| x.1)
    }

    fn replace(&mut self, dense_index: DenseIndex, value: T) -> T {
        let old = self.get_mut(dense_index);
        std::mem::replace(old, value)
    }

    fn swap_remove(&mut self, dense_index: DenseIndex) -> T {
        let item = self.items.swap_remove(dense_index.index());
        item.0
    }

    // internal sort, the outer container must reset sparse indices
    fn sort(&mut self) {
        self.items.sort_unstable_by_key(|x| x.1);
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

type Item<T> = (T, SparseIndex);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sparse = SparseVec::new(2);
        assert_eq!(sparse.len(), 0);
        assert_eq!(sparse.get(0), None);

        sparse.insert(0, 0);
        assert_eq!(sparse.len(), 1);

        sparse.insert(1, 1);
        assert_eq!(sparse.len(), 2);

        assert_eq!(sparse.get(0), Some(&0));
        assert_eq!(sparse.get(1), Some(&1));
    }

    #[test]
    fn test_remove() {
        let mut sparse = SparseVec::new(2);
        assert_eq!(sparse.len(), 0);
        assert_eq!(sparse.get(0), None);

        sparse.insert(0, 0);
        sparse.insert(1, 1);

        assert_eq!(sparse.remove(0), Some(0));
        assert_eq!(sparse.len(), 1);

        assert_eq!(sparse.remove(1), Some(1));
        assert_eq!(sparse.len(), 0);
    }

    #[test]
    fn test_get_mut() {
        let mut sparse = SparseVec::new(2);
        assert_eq!(sparse.len(), 0);
        assert_eq!(sparse.get(0), None);
        assert_eq!(sparse.get_mut(0), None);

        sparse.insert(0, 0);
        sparse.insert(1, 1);

        assert_eq!(sparse.get(0), Some(&0));
        assert_eq!(sparse.get_mut(0), Some(&mut 0));

        assert_eq!(sparse.get(1), Some(&1));
        assert_eq!(sparse.get_mut(1), Some(&mut 1));

        sparse.remove(0);
        assert_eq!(sparse.get(0), None);
        assert_eq!(sparse.get_mut(0), None);

        assert_eq!(sparse.get(1), Some(&1));
        assert_eq!(sparse.get_mut(1), Some(&mut 1));

        sparse.insert(0, 0);
        sparse.remove(1);

        assert_eq!(sparse.get(0), Some(&0));
        assert_eq!(sparse.get_mut(0), Some(&mut 0));
        assert_eq!(sparse.get(1), None);
        assert_eq!(sparse.get_mut(1), None);
    }
}
