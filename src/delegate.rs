use crate::app;
use crate::app::AppId;
use crate::config;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, ManagedVecItem)]
pub struct Delegation<M: ManagedTypeApi> {
    pub id: TokenIdentifier<M>,
    pub nonce: u64,
    pub segment: ManagedBuffer<M>,
}

#[multiversx_sc::module]
pub trait DelegateModule: config::ConfigModule + app::AppModule {
    #[payable("*")]
    #[endpoint(delegate)]
    fn delegate_endpoint(&self, app_id: AppId, segment: ManagedBuffer, user: OptionalValue<ManagedAddress>) {
        let transfers = self.call_value().all_esdt_transfers();
        let delegator = user.into_option().unwrap_or_else(|| self.blockchain().get_caller());

        require!(!transfers.is_empty(), "no delegations provided");
        require!(!segment.is_empty(), "invalid segment");
        self.require_app_exists(app_id);

        let app_info = self.app_info(app_id).get();

        if self.blockchain().is_smart_contract(&app_info.manager) {
            require!(delegator == app_info.manager, "must delegate via app");
        }

        for transfer in transfers.iter() {
            self.delegate_nft(app_id, &delegator, transfer, segment.clone());
        }
    }

    #[endpoint(undelegate)]
    fn undelegate_endpoint(&self, app_id: AppId, collection: TokenIdentifier, nonce: u64) {
        self.require_app_exists(app_id);
        let caller = self.blockchain().get_caller();
        let user = self.users().get_user_id(&caller);
        let mut delegations = self.delegations(app_id).get(&user).unwrap_or_default();
        let index = delegations.iter().position(|d| d.id == collection && d.nonce == nonce);
        require!(index.is_some(), "delegation not found");

        delegations.remove(index.unwrap());

        self.delegations(app_id).insert(user, delegations);

        self.send().direct_esdt(&caller, &collection, nonce, &BigUint::from(1u8));

        self.process_app_undelegate(app_id, caller, collection, nonce);
    }

    #[view(getDelegations)]
    fn get_delegations_view(&self, app_id: AppId) -> MultiValueEncoded<Delegation<Self::Api>> {
        let mut delegations_multi = MultiValueEncoded::new();

        for (_, delegations) in self.delegations(app_id).iter() {
            for delegation in delegations.iter() {
                delegations_multi.push(delegation);
            }
        }

        delegations_multi
    }

    #[view(getDelegationsByUser)]
    fn get_delegations_by_user_view(&self, app_id: AppId, address: ManagedAddress) -> MultiValueEncoded<Delegation<Self::Api>> {
        let user = self.users().get_user_id(&address);
        let delegations = self.delegations(app_id).get(&user).unwrap_or_default();

        MultiValueEncoded::from(delegations)
    }

    fn delegate_nft(&self, app_id: AppId, delegator: &ManagedAddress, nft: EsdtTokenPayment, segment: ManagedBuffer) {
        require!(nft.token_nonce != 0, "must not be fungible");
        require!(nft.amount == 1, "must be single nft");

        self.require_app_exists(app_id);
        let app_info = self.app_info(app_id).get();

        let is_allowed_collection = app_info.data_collections.contains(&nft.token_identifier);
        require!(is_allowed_collection, "collection not allowed for app");

        let user = self.users().get_or_create_user(delegator);
        let mut delegations = self.delegations(app_id).get(&user).unwrap_or_default();

        delegations.push(Delegation {
            id: nft.token_identifier,
            nonce: nft.token_nonce,
            segment,
        });

        self.delegations(app_id).insert(user, delegations);
    }
}
