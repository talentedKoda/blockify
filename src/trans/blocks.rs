use serde::{Deserialize, Serialize};

use crate::{
    axs::dat::{BlockRange, TimeStamp},
    sec::merkle::MerkleTree,
};

use super::record::{Record, SignedRecord};
use crate::sec::crypto::*;

pub struct BlockError {}

pub struct ChainedInstance {
    nonce: u64,
    position: u64,
    time_stamp: TimeStamp,
    hash: Hash,
    prev_hash: Hash,
    merkle_root: Hash,
    records_range: BlockRange,
}

impl ChainedInstance {
    pub fn new(
        nonce: u64,
        position: u64,
        time_stamp: TimeStamp,
        hash: Hash,
        prev_hash: Hash,
        merkle_root: Hash,
        range: BlockRange,
    ) -> Self {
        Self {
            nonce,
            position,
            time_stamp,
            hash,
            prev_hash,
            merkle_root,
            records_range: range,
        }
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn prev_hash(&self) -> &Hash {
        &self.prev_hash
    }

    pub fn merkle_root(&self) -> &Hash {
        &self.merkle_root
    }

    pub fn time_stamp(&self) -> TimeStamp {
        self.time_stamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn position(&self) -> u64 {
        self.position
    }

    pub fn records_range(&self) -> BlockRange {
        self.records_range
    }

    pub fn records<R: Record>(&self) -> Result<Vec<SignedRecord<R>>, BlockError> {
        unimplemented!()
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Hash)]
pub struct Block<R> {
    records: Vec<SignedRecord<R>>,
    merkle: MerkleTree,
    merkle_root: Hash,
}

impl<R: Record> Block<R> {
    pub fn merkle_root(&self) -> &Hash {
        &self.merkle_root
    }

    pub fn push(&mut self, item: SignedRecord<R>) -> Result<(), BlockError> {
        let hash = item.hash();
        self.merkle.push(hash);
        self.records.push(item);
        Ok(())
    }

    pub fn records(&self) -> &Vec<SignedRecord<R>> {
        &self.records
    }
}
