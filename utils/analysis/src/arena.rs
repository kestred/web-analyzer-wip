use rustc_hash::FxHashMap;
use parking_lot::Mutex;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub trait ArenaId {
    fn into_arena_id(id: u32) -> Self;
    fn from_arena_id(self) -> u32;
}

#[macro_export]
macro_rules! impl_arena_id {
    ($name:ident) => {
        impl $crate::ArenaId for $name {
            fn into_arena_id(id: u32) -> Self {
                $name(id)
            }
            fn from_arena_id(self) -> u32 {
                self.0
            }
        }
    };
}

#[derive(Default)]
pub struct Interner<Id, Data>
where
    Id: Clone + ArenaId,
    Data: Clone + Eq + Hash,
{
    map: Mutex<Store<Id, Data>>,
}

impl<Id, Data> Interner<Id, Data>
where
    Id: Clone + ArenaId,
    Data: Clone + Eq + Hash,
{
    pub fn len(&self) -> usize {
        self.map.lock().len()
    }
    pub fn get(&self, id: Id) -> Data {
        self.map.lock().get(id)
    }
    pub fn put(&self, data: &Data) -> Id {
        self.map.lock().put(data)
    }
}

#[derive(Default)]
struct Store<Id, Data>
where
    Id: Clone + ArenaId,
    Data: Clone + Eq + Hash,
{
    arena: Arena<Id, Data>,
    index: FxHashMap<Data, Id>,
}

impl<Id, Data> Store<Id, Data>
where
    Id: Clone + ArenaId,
    Data: Clone + Eq + Hash,
{
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn get(&self, id: Id) -> Data {
        self.arena[id].clone()
    }

    pub fn put(&mut self, data: &Data) -> Id {
        match self.index.get(data) {
            Some(id) => return id.clone(),
            None => (),
        }
        let id = self.arena.alloc(data.clone());
        self.index.insert(data.clone(), id.clone());
        id
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Arena<Id: ArenaId, T> {
    data: Vec<T>,
    _ty: PhantomData<Id>,
}

impl<Id: ArenaId, T> Arena<Id, T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn alloc(&mut self, value: T) -> Id {
        let id = ArenaId::into_arena_id(self.data.len() as u32);
        self.data.push(value);
        id
    }
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.data.iter().enumerate().map(|(idx, value)| (Id::into_arena_id(idx as u32), value))
    }
}

impl<Id: ArenaId, T: fmt::Debug> fmt::Debug for Arena<Id, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Arena").field("len", &self.len()).field("data", &self.data).finish()
    }
}

impl<Id: ArenaId, T> Default for Arena<Id, T> {
    fn default() -> Arena<Id, T> {
        Arena { data: Vec::new(), _ty: PhantomData }
    }
}

impl<Id: ArenaId, T> Index<Id> for Arena<Id, T> {
    type Output = T;
    fn index(&self, idx: Id) -> &T {
        let idx = idx.from_arena_id() as usize;
        &self.data[idx]
    }
}

impl<Id: ArenaId, T> IndexMut<Id> for Arena<Id, T> {
    fn index_mut(&mut self, idx: Id) -> &mut T {
        let idx = idx.from_arena_id() as usize;
        &mut self.data[idx]
    }
}

impl<Id: ArenaId, T> FromIterator<T> for Arena<Id, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Arena { data: Vec::from_iter(iter), _ty: PhantomData }
    }
}
