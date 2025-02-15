use serde::{Deserialize, Serialize};

use crate::{
    priority_ops::{PriorityFullExit, PriorityOp},
    AccountId, SerialId, TokenId, ZkLinkPriorityOp, H256,
};

/// Tests the migration of `PriorityOp::eth_hash` from the `Vec<u8>` to `H256` type
mod backward_compatibility {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct OldPriorityOp {
        serial_id: SerialId,
        data: ZkLinkPriorityOp,
        deadline_block: u64,
        eth_hash: Vec<u8>,
        eth_block: u64,
    }

    fn old_value() -> OldPriorityOp {
        let operation = PriorityFullExit {
            chain_id: 0,
            account_id: AccountId(155),
            sub_account_id: Default::default(),
            initiator: Default::default(),
            exit_address: Default::default(),
            l2_source_token: TokenId(1000),
            l1_target_token: Default::default(),
            serial_id: 0,
            tx_hash: Default::default()
        };
        OldPriorityOp {
            serial_id: 12345,
            data: ZkLinkPriorityOp::FullExit(operation),
            deadline_block: 100,
            eth_hash: vec![2; 32],
            eth_block: 0,
        }
    }

    #[test]
    fn old_deserializes_to_new() {
        let old_value = old_value();
        let serialized = serde_json::to_value(old_value.clone()).unwrap();

        let new_value: PriorityOp = serde_json::from_value(serialized).unwrap();
        assert_eq!(old_value.serial_id, new_value.serial_id);
        assert_eq!(old_value.deadline_block, new_value.deadline_block);
        assert_eq!(old_value.data, new_value.data);
        assert_eq!(old_value.eth_block, new_value.eth_block);
    }

    #[test]
    fn old_serializes_the_same_as_new() {
        let old_value = old_value();
        let old_serialized = serde_json::to_value(old_value).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized.clone()).unwrap();
        let new_serialized = serde_json::to_value(new_value).unwrap();
        assert_eq!(old_serialized.to_string(), new_serialized.to_string());
    }

    #[test]
    fn new_roundtrip_unchanged() {
        let old_value = old_value();
        let old_serialized = serde_json::to_value(old_value).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
        let new_serialized = serde_json::to_value(new_value.clone()).unwrap();

        let new_value_restored: PriorityOp = serde_json::from_value(new_serialized).unwrap();
        assert_eq!(new_value_restored.serial_id, new_value.serial_id);
        assert_eq!(new_value_restored.deadline_block, new_value.deadline_block);
        assert_eq!(new_value_restored.data, new_value.data);
        assert_eq!(new_value_restored.eth_block, new_value.eth_block);
    }

    #[test]
    /// If the `PriorityOp::eth_hash` size is not 32 bytes, the deserialization
    /// will pad the bytes from the beginning
    fn short_vector_deserialization_padding() {
        let mut old_value = old_value();
        // remove the last element to shrink its size to 31
        let _ = old_value.eth_hash.pop().unwrap();

        let old_serialized = serde_json::to_value(old_value.clone()).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
        assert_eq!(new_value.data, old_value.data);
        assert_eq!(old_value.eth_hash[0], 0);
    }

    #[test]
    fn empty_vector_deserialized_into_zero_hash() {
        let mut old_value = old_value();
        old_value.eth_hash.clear();

        let eth_hash = old_value.eth_hash.clone();
        let data = old_value.data.clone();
        let old_serialized = serde_json::to_value(old_value).unwrap();

        let new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
        assert_eq!(new_value.data, data);
        assert_eq!(eth_hash, H256::zero().as_bytes().to_vec());
    }

    #[test]
    #[should_panic(expected = "33")]
    /// If the `PriorityOp::eth_hash` length is greater than 32 bytes, the deserialization cannot happen
    fn big_vector_cannot_be_deserialized() {
        let mut old_value = old_value();
        // add one more item to grow it to 33 bytes
        old_value.eth_hash.push(123);

        let old_serialized = serde_json::to_value(old_value).unwrap();

        let _new_value: PriorityOp = serde_json::from_value(old_serialized).unwrap();
    }
}
