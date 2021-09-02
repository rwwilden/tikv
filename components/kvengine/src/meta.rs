use std::{collections::HashMap, ops::Deref, str::FromStr};

use bytes::{Buf, Bytes};
use kvenginepb as pb;

use super::*;
use protobuf::{Message, RepeatedField};
use slog_global::*;

#[derive(Default, Clone)]
pub struct ShardMeta {
    pub id: u64,
    pub ver: u64,
    pub start: Vec<u8>,
    pub end: Vec<u8>,
    pub seq: u64,
    pub(crate) files: HashMap<u64, FileMeta>,

    pub properties: Properties,
    pub pre_split: Option<pb::PreSplit>,
    split: Option<pb::Split>,
    pub split_stage: pb::SplitStage,
    pub base_ts: u64,
    pub parent: Option<Box<ShardMeta>>,
}

impl ShardMeta {
    pub fn new(mut cs: pb::ChangeSet) -> Self {
        let mut snap = cs.take_snapshot();
        let mut meta = Self {
            id: cs.shard_id,
            ver: cs.shard_ver,
            start: snap.take_start(),
            end: snap.take_end(),
            seq: cs.sequence,
            properties: Properties::new().apply_pb(snap.get_properties()),
            split_stage: cs.stage,
            base_ts: snap.base_ts,
            ..Default::default()
        };
        if snap.get_split_keys().len() > 0 {
            let mut pre_split = pb::PreSplit::new();
            pre_split.keys = snap.take_split_keys();
            meta.pre_split = Some(pre_split);
        }
        for l0 in snap.get_l0_creates() {
            meta.add_file(l0.id, -1, 0, l0.get_smallest(), l0.get_biggest());
        }
        for tbl in snap.get_table_creates() {
            meta.add_file(
                tbl.id,
                tbl.cf,
                tbl.level,
                tbl.get_smallest(),
                tbl.get_biggest(),
            );
        }
        meta
    }

    pub fn new_split(
        id: u64,
        ver: u64,
        start: &[u8],
        end: &[u8],
        props: &pb::Properties,
        parent: Box<ShardMeta>,
    ) -> Self {
        Self {
            id,
            ver,
            start: Vec::from(start),
            end: Vec::from(end),
            parent: Some(parent),
            properties: Properties::new().apply_pb(props),
            ..Default::default()
        }
    }

    fn move_down_file(&mut self, id: u64, cf: i32, level: u32) {
        info!("{}:{} move down file {}", self.id, self.ver, id);
        let mut fm = self.files.get_mut(&id).unwrap();
        assert!(fm.level + 1 == level as u8);
        assert!(fm.cf == cf as i8);
        fm.level = level as u8;
    }

    fn add_file(&mut self, id: u64, cf: i32, level: u32, smallest: &[u8], biggest: &[u8]) {
        info!("{}:{} add file {}", self.id, self.ver, id);
        self.files
            .insert(id, FileMeta::new(cf, level, smallest, biggest));
    }

    fn delete_file(&mut self, id: u64) {
        info!("{}:{} delete file {}", self.id, self.ver, id);
        self.files.remove(&id);
    }

    fn file_level(&self, id: u64) -> Option<u32> {
        self.files.get(&id).map(|fm| fm.level as u32)
    }

    fn get_property(&self, key: &str) -> Option<Bytes> {
        self.properties.get(key).map(|v| v.clone())
    }

    pub fn apply_change_set(&mut self, mut cs: pb::ChangeSet) {
        if self.is_duplicated_change_set(&mut cs) {
            return;
        }
        if cs.sequence > 0 {
            self.seq = cs.sequence;
        }
        if cs.has_flush() {
            self.apply_flush(cs);
            return;
        }
        if cs.has_pre_split() {
            self.apply_pre_split(cs.take_pre_split());
            return;
        }
        if cs.has_compaction() {
            self.apply_compaction(cs.take_compaction());
            return;
        }
        if cs.has_split_files() {
            self.apply_split_files(cs.take_split_files());
            return;
        }
        panic!("unexpected change set {:?}", cs)
    }

    pub fn is_duplicated_change_set(&self, cs: &mut pb::ChangeSet) -> bool {
        if cs.sequence > 0 && self.seq >= cs.sequence {
            info!(
                "{}:{} skip duplicated change, meta seq:{}",
                self.id, self.ver, cs.sequence
            );
            return true;
        }
        if cs.has_pre_split() {
            return self.pre_split.is_some();
        }
        if cs.has_flush() {
            return false;
        }
        if cs.has_split_files() {
            return self.split_stage == cs.stage;
        }
        if cs.has_compaction() {
            let comp = cs.mut_compaction();
            if is_move_down(&comp) {
                if let Some(level) = self.file_level(comp.get_top_deletes()[0]) {
                    if level == comp.level {
                        return false;
                    }
                }
                info!(
                    "{}:{} skip duplicated move_down compaction level:{}",
                    self.id, self.ver, comp.level
                );
                return true;
            }
            for i in 0..comp.get_top_deletes().len() {
                let id = comp.get_top_deletes()[i];
                if self.is_compaction_file_deleted(id, comp) {
                    return true;
                }
            }
            for i in 0..comp.get_bottom_deletes().len() {
                let id = comp.get_bottom_deletes()[i];
                if self.is_compaction_file_deleted(id, comp) {
                    return true;
                }
            }
        }
        false
    }

    fn is_compaction_file_deleted(&self, id: u64, comp: &mut pb::Compaction) -> bool {
        if !self.files.contains_key(&id) {
            info!(
                "{}:{} skip duplicated compaction file {} already deleted.",
                self.id, self.ver, id
            );
            comp.conflicted = true;
            return true;
        }
        return false;
    }

    fn apply_flush(&mut self, cs: pb::ChangeSet) {
        let flush = cs.get_flush();
        self.parent = None;
        let props = flush.get_properties();
        for i in 0..props.get_keys().len() {
            let key = &props.get_keys()[i];
            let val = &props.get_values()[i];
            self.properties.set(key, val.as_slice());
        }
        if cs.stage == pb::SplitStage::PreSplitFlushDone {
            self.split_stage = pb::SplitStage::PreSplitFlushDone;
        }
        if flush.has_l0_create() {
            let l0 = flush.get_l0_create();
            self.add_file(l0.id, -1, 0, l0.get_smallest(), l0.get_biggest());
        }
    }

    fn apply_pre_split(&mut self, pre_split: pb::PreSplit) {
        self.pre_split = Some(pre_split);
        self.split_stage = pb::SplitStage::PreSplit;
    }

    fn apply_compaction(&mut self, comp: pb::Compaction) {
        if is_move_down(&comp) {
            for tbl in comp.get_table_creates() {
                self.move_down_file(tbl.id, tbl.cf, tbl.level);
            }
            return;
        }
        for id in comp.get_top_deletes() {
            self.delete_file(*id);
        }
        for id in comp.get_bottom_deletes() {
            self.delete_file(*id);
        }
        for tbl in comp.get_table_creates() {
            self.add_file(
                tbl.id,
                tbl.cf,
                tbl.level,
                tbl.get_smallest(),
                tbl.get_biggest(),
            )
        }
    }

    fn apply_split_files(&mut self, split_files: pb::SplitFiles) {
        for id in split_files.get_table_deletes() {
            self.files.remove(id);
        }
        for l0 in split_files.get_table_creates() {
            self.add_file(l0.id, -1, 0, l0.get_smallest(), l0.get_biggest());
        }
        for tbl in split_files.get_table_creates() {
            self.add_file(
                tbl.id,
                tbl.cf,
                tbl.level,
                tbl.get_smallest(),
                tbl.get_biggest(),
            );
        }
        self.split_stage = pb::SplitStage::SplitFileDone;
    }

    pub fn apply_split(&mut self, cs: pb::ChangeSet) -> Vec<ShardMeta> {
        let old = self;
        let split = cs.get_split();
        let new_shards_len = split.get_new_shards().len();
        let mut new_shards = Vec::with_capacity(new_shards_len);
        let new_ver = old.ver + new_shards_len as u64 - 1;
        for i in 0..new_shards_len {
            let (start_key, end_key) =
                get_splitting_start_end(&old.start, &old.end, split.get_keys(), i);
            let new_shard = &split.get_new_shards()[i];
            let id = new_shard.get_shard_id();
            let mut meta = ShardMeta::new_split(
                id,
                new_ver,
                start_key,
                end_key,
                new_shard,
                Box::new(old.clone()),
            );
            if id == old.id {
                old.split = Some(split.clone());
                meta.base_ts = old.base_ts;
                meta.seq = old.seq;
            } else {
                meta.base_ts = old.base_ts + cs.sequence;
                meta.seq = 1;
            }
            new_shards.push(meta);
        }
        for (fid, fm) in &old.files {
            let shard_idx = get_split_shard_index(split.get_keys(), fm.smallest.chunk());
            new_shards[shard_idx].files.insert(*fid, fm.clone());
        }
        new_shards
    }

    pub fn to_change_set(&self) -> pb::ChangeSet {
        let mut cs = new_change_set(self.id, self.ver, self.split_stage);
        cs.set_sequence(self.seq);
        let mut snap = pb::Snapshot::new();
        snap.set_start(self.start.clone());
        snap.set_end(self.end.clone());
        snap.set_properties(self.properties.to_pb(self.id));
        if let Some(pre_split) = &self.pre_split {
            snap.set_split_keys(RepeatedField::from_slice(pre_split.get_keys()));
        }
        for (k, v) in self.files.iter() {
            if v.level == 0 {
                let mut l0 = pb::L0Create::new();
                l0.set_id(*k);
                l0.set_smallest(v.smallest.to_vec());
                l0.set_biggest(v.biggest.to_vec());
                snap.mut_l0_creates().push(l0);
            } else {
                let mut tbl = pb::TableCreate::new();
                tbl.set_id(*k);
                tbl.set_cf(v.cf as i32);
                tbl.set_level(v.level as u32);
                tbl.set_smallest(v.smallest.to_vec());
                tbl.set_biggest(v.biggest.to_vec());
                snap.mut_table_creates().push(tbl);
            }
        }
        cs.set_snapshot(snap);
        if let Some(parent) = &self.parent {
            cs.set_parent(parent.to_change_set());
        }
        cs
    }

    pub fn marshal(&self) -> Vec<u8> {
        let cs = self.to_change_set();
        cs.write_to_bytes().unwrap()
    }
}

#[derive(Clone)]
pub(crate) struct FileMeta {
    pub(crate) cf: i8,
    pub(crate) level: u8,
    pub(crate) smallest: Bytes,
    pub(crate) biggest: Bytes,
}

impl FileMeta {
    fn new(cf: i32, level: u32, smallest: &[u8], biggest: &[u8]) -> Self {
        Self {
            cf: cf as i8,
            level: level as u8,
            smallest: Bytes::copy_from_slice(smallest),
            biggest: Bytes::copy_from_slice(biggest),
        }
    }
}

pub fn is_move_down(comp: &pb::Compaction) -> bool {
    comp.top_deletes.len() == comp.table_creates.len()
        && comp.top_deletes[0] == comp.table_creates[0].id
}

pub fn new_change_set(shard_id: u64, shard_ver: u64, stage: pb::SplitStage) -> pb::ChangeSet {
    let mut cs = pb::ChangeSet::new();
    cs.set_shard_id(shard_id);
    cs.set_shard_ver(shard_ver);
    cs.set_stage(stage);
    cs
}

pub const GLOBAL_SHARD_END_KEY: Bytes =
    Bytes::from_static(&[255, 255, 255, 255, 255, 255, 255, 255]);

trait MetaReader {
    fn iterate_meta<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&pb::ChangeSet) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta() {
        println!("dd")
    }
}
