// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    strategy_burn_token
    (
        init => init
        upgrade => upgrade
        registerApp => register_app_endpoint
        participate => participate_endpoint
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
