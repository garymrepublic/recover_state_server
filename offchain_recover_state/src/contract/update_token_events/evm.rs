use async_trait::async_trait;
use ethers::contract::Contract;
use ethers::prelude::{Address, Http, Provider};
use zklink_types::{ChainId, Token};
use ethers::core::types::BlockNumber as EthBlockNumber;
use tokio::sync::mpsc::Sender;
use zklink_blockchain::eth::ZkLinkAddressWrapper;
use recover_state_config::Layer1Config;
use zklink_storage::ConnectionPool;
use super::UpdateTokenEvents;
use crate::contract::utils::{load_abi, new_provider_with_url, NewToken, ZKLINK_JSON};
use crate::database_storage_interactor::DatabaseStorageInteractor;
use crate::storage_interactor::StorageInteractor;
use crate::VIEW_BLOCKS_STEP;

pub struct EvmTokenEvents {
    contract: Contract<Provider<Http>>,
    chain_id: ChainId,
    last_sync_block_number: u64,
    connection_pool: ConnectionPool,
}

impl EvmTokenEvents {
    pub async fn new(config: &Layer1Config, connection_pool: ConnectionPool) -> Self{
        let last_watched_block_number = {
            let mut storage = connection_pool.access_storage().await.unwrap();
            storage.recover_schema()
                .last_watched_block_number(*config.chain.chain_id as i16, "token")
                .await
                .expect("Failed to get last watched block number")
                .map(|num| num as u64)
        };
        let address = Address::from_slice(config.contracts.contract_addr.as_bytes());
        let abi = load_abi(ZKLINK_JSON);
        let client = new_provider_with_url(&config.client.web3_url());

        Self{
            contract: Contract::new(address, abi, client.into()),
            chain_id: config.chain.chain_id,
            last_sync_block_number: last_watched_block_number.unwrap_or(config.contracts.deployment_block),
            connection_pool,
        }
    }
}

#[async_trait]
impl UpdateTokenEvents for EvmTokenEvents {
    async fn update_token_events(&mut self, token_sender: Sender<Token>) -> anyhow::Result<u64> {
        let from = self.last_sync_block_number;
        let to = self.last_sync_block_number + VIEW_BLOCKS_STEP;
        let events: Vec<NewToken> = self.contract
            .event_for_name("NewToken")?
            .from_block(EthBlockNumber::Number(from.into()))
            .to_block(EthBlockNumber::Number(to.into()))
            .query()
            .await?;

        let mut interactor = {
            let storage = self.connection_pool.access_storage().await?;
            DatabaseStorageInteractor::new(storage)
        };
        interactor.store_tokens(&events, self.chain_id).await;
        interactor.update_token_event_progress(self.chain_id,to).await;

        for event in events{
            token_sender.send(
                Token{
                    id: event.id.into(),
                    chains: vec![self.chain_id]
                }
            ).await?;
        }
        self.last_sync_block_number = to;

        Ok(to)
    }
}