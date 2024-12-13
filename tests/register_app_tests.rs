use multiversx_sc::types::*;
use multiversx_sc_scenario::api::*;
use setup::*;

pub mod setup;

#[test]
fn it_registers_an_app() {
    let mut contract = TestContract::new();

    let burn_token = TokenIdentifier::<StaticApi>::from("BURN-123456");

    contract.register_app(burn_token);

    assert!(true); // TODO: figure out how to check storage in type safe way
}

// TODO: it_fails_when_app_registered_already
