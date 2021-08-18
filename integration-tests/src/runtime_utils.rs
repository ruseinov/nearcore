use std::collections::HashSet;

use near_chain_configs::Genesis;
use near_primitives::state_record::{state_record_to_account_id, StateRecord};
use near_primitives::types::AccountId;
use near_primitives::types::StateRoot;
use near_store::test_utils::create_tries;
use near_store::{ShardTries, TrieUpdate};
use nearcore::config::GenesisExt;
use node_runtime::{state_viewer::TrieViewer, Runtime};
use testlib::runtime_utils::{add_test_contract, alice_account, bob_account};

pub fn get_runtime_and_trie() -> (Runtime, ShardTries, StateRoot) {
    let mut genesis =
        Genesis::test(vec![alice_account(), bob_account(), "carol.near".parse().unwrap()], 3);
    add_test_contract(&mut genesis, &"test.contract".parse().unwrap());
    get_runtime_and_trie_from_genesis(&genesis)
}

pub fn get_test_trie_viewer() -> (TrieViewer, TrieUpdate) {
    let (_, tries, root) = get_runtime_and_trie();
    let trie_viewer = TrieViewer::default();
    let state_update = tries.new_trie_update(0, root);
    (trie_viewer, state_update)
}

pub fn get_runtime_and_trie_from_genesis(genesis: &Genesis) -> (Runtime, ShardTries, StateRoot) {
    let tries = create_tries();
    let runtime = Runtime::new();
    let mut account_ids: HashSet<AccountId> = HashSet::new();
    genesis.for_each_record(|record: &StateRecord| {
        account_ids.insert(state_record_to_account_id(record).clone());
    });
    let genesis_root = runtime.apply_genesis_state(
        tries.clone(),
        0,
        &genesis
            .config
            .validators
            .iter()
            .map(|account_info| {
                (
                    account_info.account_id.clone(),
                    account_info.public_key.clone(),
                    account_info.amount,
                )
            })
            .collect::<Vec<_>>(),
        &genesis,
        &genesis.config.runtime_config,
        account_ids,
    );
    (runtime, tries, genesis_root)
}