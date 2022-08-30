// Copyright 2022 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use futures::executor::block_on;
use kvproto::metapb::{Peer, PeerRole};
use pd_client::PdClient;
use rand::Rng;
use test_raftstore::TestPdClient;

use crate::{must_wait, try_wait};

pub struct Scheduler {
    pub(crate) pd: Arc<TestPdClient>,
    pub(crate) store_ids: Vec<u64>,
    pub(crate) lock: Arc<DashMap<u64, Arc<Mutex<()>>>>,
}

impl Scheduler {
    pub fn move_random_region(&self) {
        let regions = self.pd.get_all_regions();
        let region_idx = rand::thread_rng().gen_range(0..regions.len());
        let region = &regions[region_idx];
        if region.get_peers().len() != 3 {
            return;
        }
        let &target_store_id = self
            .store_ids
            .iter()
            .find(|&&store_id| {
                let contains = region
                    .get_peers()
                    .iter()
                    .any(|peer| peer.store_id == store_id);
                !contains
            })
            .unwrap();
        self.move_peer(region.id, target_store_id);
    }

    fn move_peer(&self, region_id: u64, store_id: u64) {
        let mutex = self.get_region_mutex(region_id);
        let _guard = mutex.lock().unwrap();
        let peer_id = self.pd.alloc_id().unwrap();
        let mut peer = Peer::new();
        peer.store_id = store_id;
        peer.id = peer_id;
        peer.role = PeerRole::Learner;
        self.pd.add_peer(region_id, peer);
        must_wait(
            || {
                let region = block_on(self.pd.get_region_by_id(region_id))
                    .unwrap()
                    .unwrap();
                region.get_peers().iter().any(|peer| peer.id == peer_id)
            },
            10,
            "failed to add learner",
        );
        let mut peer = Peer::new();
        peer.store_id = store_id;
        peer.id = peer_id;
        peer.role = PeerRole::Voter;
        self.pd.add_peer(region_id, peer);
        must_wait(
            || {
                let region = block_on(self.pd.get_region_by_id(region_id))
                    .unwrap()
                    .unwrap();
                region
                    .get_peers()
                    .iter()
                    .any(|peer| peer.id == peer_id && peer.role == PeerRole::Voter)
            },
            10,
            "failed to promote learner",
        );
        must_wait(
            || {
                block_on(self.pd.get_region_leader_by_id(region_id))
                    .unwrap()
                    .map(|(region, leader)| {
                        let to_remove = region
                            .peers
                            .iter()
                            .find(|peer| peer.id != leader.id)
                            .unwrap();
                        self.pd.try_remove_peer(region_id, to_remove.clone());
                        try_wait(
                            || {
                                let region = block_on(self.pd.get_region_by_id(region_id))
                                    .unwrap()
                                    .unwrap();
                                region.get_peers().len() == 3
                            },
                            3,
                        )
                    })
                    .unwrap_or(false)
            },
            15,
            "failed to remove peer",
        );
    }

    pub fn transfer_random_leader(&self) -> bool {
        let regions = self.pd.get_all_regions();
        if regions.len() < 3 {
            return false;
        }
        let region_idx = rand::thread_rng().gen_range(0..regions.len());
        let region = &regions[region_idx];
        if region.get_peers().len() != 3 {
            return false;
        }
        let region_id = region.get_id();
        let mutex = self.get_region_mutex(region_id);
        let _guard = mutex.lock().unwrap();
        block_on(self.pd.get_region_leader_by_id(region_id))
            .unwrap()
            .map(|(_, leader)| {
                let old_leader_id = leader.id;
                region
                    .peers
                    .iter()
                    .find(|peer| peer.id != old_leader_id && peer.role == PeerRole::Voter)
                    .map(|target| {
                        self.pd.try_transfer_leader(region_id, target.clone());
                        let new_leader_id = target.id;
                        try_wait(
                            || {
                                block_on(self.pd.get_region_leader_by_id(region_id))
                                    .unwrap()
                                    .map(|(_, leader)| leader.id == new_leader_id)
                                    .unwrap_or(false)
                            },
                            3,
                        )
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    pub fn get_region_mutex(&self, region_id: u64) -> Arc<Mutex<()>> {
        self.lock
            .entry(region_id)
            .or_insert(Arc::new(Mutex::default()))
            .clone()
    }
}