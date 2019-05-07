use rustc_hash::FxHashMap;
use parking_lot::Mutex;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub trait IdType {
    fn into_id_type(id: u32) -> Self;
    fn from_id_type(self) -> u32;
}

/// e.g. `id!(pub struct ExampleId);`
#[macro_export]
macro_rules! impl_id_type {
    ($name:ident) => {
        $vis struct $name(u32);
        impl $crate::IdType for $name {
            fn into_id_type(id: u32) -> Self {
                $name(id)
            }
            fn from_id_type(self) -> u32 {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_intern_key {
    ($name:ident) => {
        impl salsa::InternKey for $name {
            fn from_intern_id(v: salsa::InternId) -> Self {
                $name(v)
            }
            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    }
}

#[derive(Default)]
pub struct Interner<Id, Data>
where
    Id: Clone + IdType,
    Data: Clone + Eq + Hash,
{
    map: Mutex<Store<Id, Data>>,
}

impl<Id, Data> Interner<Id, Data>
where
    Id: Clone + IdType,
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
    Id: Clone + IdType,
    Data: Clone + Eq + Hash,
{
    arena: Arena<Id, Data>,
    index: FxHashMap<Data, Id>,
}

impl<Id, Data> Store<Id, Data>
where
    Id: Clone + IdType,
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
pub struct Arena<Id: IdType, T> {
    data: Vec<T>,
    _ty: PhantomData<Id>,
}

impl<Id: IdType, T> Arena<Id, T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn alloc(&mut self, value: T) -> Id {
        let id = IdType::into_id_type(self.data.len() as u32);
        self.data.push(value);
        id
    }
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.data.iter().enumerate().map(|(idx, value)| (Id::into_id_type(idx as u32), value))
    }
}

impl<Id: IdType, T> Default for Arena<Id, T> {
    fn default() -> Arena<Id, T> {
        Arena { data: Vec::new(), _ty: PhantomData }
    }
}

impl<Id: IdType, T> Index<Id> for Arena<Id, T> {
    type Output = T;
    fn index(&self, idx: Id) -> &T {
        let idx = idx.from_id_type() as usize;
        &self.data[idx]
    }
}

impl<Id: IdType, T> IndexMut<Id> for Arena<Id, T> {
    fn index_mut(&mut self, idx: Id) -> &mut T {
        let idx = idx.from_id_type() as usize;
        &mut self.data[idx]
    }
}

impl<Id: IdType, T> FromIterator<T> for Arena<Id, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Arena { data: Vec::from_iter(iter), _ty: PhantomData }
    }
}
