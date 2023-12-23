multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[endpoint(addDataCollectionWhitelist)]
    fn add_data_collection_whitelist(&self, token_identifier: TokenIdentifier) {
        self.data_collection_whitelist().insert(token_identifier);
    }

    #[storage_mapper("config:data_collection_whitelist")]
    fn data_collection_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
