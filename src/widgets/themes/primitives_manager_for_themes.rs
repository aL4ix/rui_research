use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

use std::collections::btree_map::IterMut;

use crate::widgets::Primitive;

pub type PrimId = usize;

pub trait PrimEnum: Clone + Copy + Debug {
    fn to_prim_id(self) -> PrimId;
}

type Prims = BTreeMap<isize, Box<dyn Primitive>>;
type PrimManPidRid = HashMap<PrimId, isize>;

#[derive(Debug, Default)]
pub struct PrimitivesManagerForThemes {
    prims: Prims,
    pid_rid: PrimManPidRid,
}

impl PrimitivesManagerForThemes {
    const NOT_FOUND: &str = "Could not find prim_enum in pid_rid map:";
    pub fn new() -> Self {
        PrimitivesManagerForThemes {
            prims: Default::default(),
            pid_rid: Default::default(),
        }
    }
    pub fn insert<K: PrimEnum, V: Primitive>(&mut self, prim_enum: K, prim: V, render_id: isize) {
        self.pid_rid.insert(prim_enum.to_prim_id(), render_id);
        self.prims.insert(render_id, Box::new(prim));
    }
    pub fn get_mut<K: PrimEnum>(&mut self, prim_enum: K) -> Option<&mut Box<dyn Primitive>> {
        let rid = self
            .pid_rid
            .get(&prim_enum.to_prim_id())
            .unwrap_or_else(|| panic!("{} {:?}", Self::NOT_FOUND, prim_enum));
        self.prims.get_mut(rid)
    }
    pub fn remove<K: PrimEnum>(&mut self, prim_enum: K) -> Option<(isize, Box<dyn Primitive>)> {
        let rid = self
            .pid_rid
            .get(&prim_enum.to_prim_id())
            .unwrap_or_else(|| panic!("{} {:?}", Self::NOT_FOUND, prim_enum));
        self.prims.remove(rid).map(|p| (*rid, p))
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, isize, Box<dyn Primitive>> {
        self.prims.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.prims.len()
    }
    pub fn is_empty(&self) -> bool {
        self.prims.is_empty()
    }
}
