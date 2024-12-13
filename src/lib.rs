#![no_std]

use errors::{ERR_APP_NOT_REGISTERED, ERR_APP_REGISTERED_ALREADY, ERR_PAYMENT_ZERO, ERR_TOKEN_INVALID, ERR_TOKEN_INVALID_ID};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod errors;
pub mod strategy_burn_token_proxy;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct AppInfo<M: ManagedTypeApi> {
    pub burn_token: TokenIdentifier<M>,
}

#[multiversx_sc::contract]
pub trait StrategyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(registerApp)]
    fn register_app_endpoint(&self, burn_token: TokenIdentifier) {
        require!(burn_token.is_valid_esdt_identifier(), ERR_TOKEN_INVALID_ID);

        let app = self.blockchain().get_caller();
        require!(self.app_infos(&app).is_empty(), ERR_APP_REGISTERED_ALREADY);
    }

    #[payable("*")]
    #[endpoint(participate)]
    fn participate_endpoint(&self, app: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let app_info = self.get_app_info_or_fail(&app);

        let payment = self.call_value().single_esdt();
        require!(payment.amount > 0, ERR_PAYMENT_ZERO);
        require!(payment.token_identifier == app_info.burn_token, ERR_TOKEN_INVALID);

        self.tx()
            .to(ToSelf)
            .typed(system_proxy::UserBuiltinProxy)
            .esdt_local_burn(&app_info.burn_token, 0, &payment.amount)
            .sync_call();

        let existing_weight = self.members(&app).get(&caller).unwrap_or_default();
        let new_weight = &existing_weight + &payment.amount;

        self.members(&app).insert(caller.clone(), new_weight);
    }

    fn get_app_info_or_fail(&self, address: &ManagedAddress) -> AppInfo<Self::Api> {
        require!(!self.app_infos(address).is_empty(), ERR_APP_NOT_REGISTERED);

        self.app_infos(address).get()
    }

    #[storage_mapper("members")]
    fn members(&self, address: &ManagedAddress) -> MapMapper<ManagedAddress, BigUint>;

    #[storage_mapper("app_infos")]
    fn app_infos(&self, address: &ManagedAddress) -> SingleValueMapper<AppInfo<Self::Api>>;
}
