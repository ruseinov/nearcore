mod challenges;
mod simple_chain;

use crate::types::Tip;
use crate::{Block, Chain, Error, Provenance};
use near_primitives::account::id::AccountId;
impl Chain {
    /// A wrapper function around process_block that doesn't trigger all the callbacks
    /// Only used in tests
    pub(crate) fn process_block_test(
        &mut self,
        me: &Option<AccountId>,
        block: Block,
    ) -> Result<Option<Tip>, Error> {
        self.process_block(me, block, Provenance::PRODUCED, |_| {}, |_| {}, |_| {})
    }
}