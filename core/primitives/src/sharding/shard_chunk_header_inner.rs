use crate::types::validator_stake::{ValidatorStake, ValidatorStakeIter, ValidatorStakeV1};
use crate::types::StateRoot;
use borsh::{BorshDeserialize, BorshSerialize};
use near_primitives_core::hash::CryptoHash;
use near_primitives_core::types::{Balance, BlockHeight, Gas, ShardId};

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum ShardChunkHeaderInner {
    V1(ShardChunkHeaderInnerV1),
    V2(ShardChunkHeaderInnerV2),
    V3(ShardChunkHeaderInnerV3),
}

impl ShardChunkHeaderInner {
    #[inline]
    pub fn prev_state_root(&self) -> &StateRoot {
        match self {
            Self::V1(inner) => &inner.prev_state_root,
            Self::V2(inner) => &inner.prev_state_root,
            Self::V3(inner) => &inner.prev_state_root,
        }
    }

    #[inline]
    pub fn prev_block_hash(&self) -> &CryptoHash {
        match self {
            Self::V1(inner) => &inner.prev_block_hash,
            Self::V2(inner) => &inner.prev_block_hash,
            Self::V3(inner) => &inner.prev_block_hash,
        }
    }

    #[inline]
    pub fn gas_limit(&self) -> Gas {
        match self {
            Self::V1(inner) => inner.gas_limit,
            Self::V2(inner) => inner.gas_limit,
            Self::V3(inner) => inner.gas_limit,
        }
    }

    #[inline]
    pub fn prev_gas_used(&self) -> Gas {
        match self {
            Self::V1(inner) => inner.prev_gas_used,
            Self::V2(inner) => inner.prev_gas_used,
            Self::V3(inner) => inner.prev_gas_used,
        }
    }

    #[inline]
    pub fn prev_validator_proposals(&self) -> ValidatorStakeIter {
        match self {
            Self::V1(inner) => ValidatorStakeIter::v1(&inner.prev_validator_proposals),
            Self::V2(inner) => ValidatorStakeIter::new(&inner.prev_validator_proposals),
            Self::V3(inner) => ValidatorStakeIter::new(&inner.prev_validator_proposals),
        }
    }

    #[inline]
    pub fn height_created(&self) -> BlockHeight {
        match self {
            Self::V1(inner) => inner.height_created,
            Self::V2(inner) => inner.height_created,
            Self::V3(inner) => inner.height_created,
        }
    }

    #[inline]
    pub fn shard_id(&self) -> ShardId {
        match self {
            Self::V1(inner) => inner.shard_id,
            Self::V2(inner) => inner.shard_id,
            Self::V3(inner) => inner.shard_id,
        }
    }

    #[inline]
    pub fn prev_outcome_root(&self) -> &CryptoHash {
        match self {
            Self::V1(inner) => &inner.prev_outcome_root,
            Self::V2(inner) => &inner.prev_outcome_root,
            Self::V3(inner) => &inner.prev_outcome_root,
        }
    }

    #[inline]
    pub fn encoded_merkle_root(&self) -> &CryptoHash {
        match self {
            Self::V1(inner) => &inner.encoded_merkle_root,
            Self::V2(inner) => &inner.encoded_merkle_root,
            Self::V3(inner) => &inner.encoded_merkle_root,
        }
    }

    #[inline]
    pub fn encoded_length(&self) -> u64 {
        match self {
            Self::V1(inner) => inner.encoded_length,
            Self::V2(inner) => inner.encoded_length,
            Self::V3(inner) => inner.encoded_length,
        }
    }

    #[inline]
    pub fn prev_balance_burnt(&self) -> Balance {
        match self {
            Self::V1(inner) => inner.prev_balance_burnt,
            Self::V2(inner) => inner.prev_balance_burnt,
            Self::V3(inner) => inner.prev_balance_burnt,
        }
    }

    #[inline]
    pub fn prev_outgoing_receipts_root(&self) -> &CryptoHash {
        match self {
            Self::V1(inner) => &inner.prev_outgoing_receipts_root,
            Self::V2(inner) => &inner.prev_outgoing_receipts_root,
            Self::V3(inner) => &inner.prev_outgoing_receipts_root,
        }
    }

    #[inline]
    pub fn tx_root(&self) -> &CryptoHash {
        match self {
            Self::V1(inner) => &inner.tx_root,
            Self::V2(inner) => &inner.tx_root,
            Self::V3(inner) => &inner.tx_root,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShardChunkHeaderInnerV1 {
    /// Previous block hash.
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: StateRoot,
    /// Root of the outcomes from execution transactions and results of the previous chunk.
    pub prev_outcome_root: CryptoHash,
    pub encoded_merkle_root: CryptoHash,
    pub encoded_length: u64,
    pub height_created: BlockHeight,
    /// Shard index.
    pub shard_id: ShardId,
    /// Gas used in the previous chunk.
    pub prev_gas_used: Gas,
    /// Gas limit voted by validators.
    pub gas_limit: Gas,
    /// Total balance burnt in the previous chunk.
    pub prev_balance_burnt: Balance,
    /// Previous chunk's outgoing receipts merkle root.
    pub prev_outgoing_receipts_root: CryptoHash,
    /// Tx merkle root.
    pub tx_root: CryptoHash,
    /// Validator proposals from the previous chunk.
    pub prev_validator_proposals: Vec<ValidatorStakeV1>,
}

// V1 -> V2: Use versioned ValidatorStake structure in proposals
#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShardChunkHeaderInnerV2 {
    /// Previous block hash.
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: StateRoot,
    /// Root of the outcomes from execution transactions and results of the previous chunk.
    pub prev_outcome_root: CryptoHash,
    pub encoded_merkle_root: CryptoHash,
    pub encoded_length: u64,
    pub height_created: BlockHeight,
    /// Shard index.
    pub shard_id: ShardId,
    /// Gas used in the previous chunk.
    pub prev_gas_used: Gas,
    /// Gas limit voted by validators.
    pub gas_limit: Gas,
    /// Total balance burnt in the previous chunk.
    pub prev_balance_burnt: Balance,
    /// Previous chunk's outgoing receipts merkle root.
    pub prev_outgoing_receipts_root: CryptoHash,
    /// Tx merkle root.
    pub tx_root: CryptoHash,
    /// Validator proposals from the previous chunk.
    pub prev_validator_proposals: Vec<ValidatorStake>,
}

// V2 -> V3: Switch to post-state-root
#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct ShardChunkHeaderInnerV3 {
    /// Previous block hash.
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: StateRoot,
    /// Root of the outcomes from execution transactions and results of the previous chunk.
    pub prev_outcome_root: CryptoHash,
    pub encoded_merkle_root: CryptoHash,
    pub encoded_length: u64,
    pub height_created: BlockHeight,
    /// Shard index.
    pub shard_id: ShardId,
    /// Gas used in the previous chunk.
    pub prev_gas_used: Gas,
    /// Gas limit voted by validators.
    pub gas_limit: Gas,
    /// Total balance burnt in the previous chunk.
    pub prev_balance_burnt: Balance,
    /// Previous chunk's outgoing receipts merkle root.
    pub prev_outgoing_receipts_root: CryptoHash,
    /// Tx merkle root.
    pub tx_root: CryptoHash,
    /// Validator proposals from the previous chunk.
    pub prev_validator_proposals: Vec<ValidatorStake>,

    /// State root after applying this chunk.
    pub post_state_root: StateRoot,
    /// Gas limit for the next chunk.
    pub next_gas_limit: Gas,
    /// Gas used in this chunk.
    pub gas_used: Gas,
    /// Validator proposals from this chunk.
    pub validator_proposals: Vec<ValidatorStake>,
    /// Root of the outcomes from execution transactions and results of this chunk.
    pub outcome_root: CryptoHash,
    /// Total balance burnt in this chunk.
    pub balance_burnt: Balance,
    /// This chunk's outgoing receipts merkle root.
    pub outgoing_receipts_root: CryptoHash,
}
