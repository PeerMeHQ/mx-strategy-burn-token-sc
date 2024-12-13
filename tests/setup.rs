use multiversx_sc_scenario::imports::*;
use strategy_burn_token::*;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const APP_ADDRESS: TestAddress = TestAddress::new("app");
pub const STRATEGY_ADDRESS: TestSCAddress = TestSCAddress::new("strategy");
pub const CODE_PATH: MxscPath = MxscPath::new("output/strategy_burn_token.mxsc.json");

pub fn blockchain() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, strategy_burn_token::ContractBuilder);

    blockchain
}

pub struct TestContract {
    pub chain: ScenarioWorld,
}

impl TestContract {
    pub fn new() -> Self {
        let chain = blockchain();
        let mut contract = Self { chain };

        contract.chain.account(APP_ADDRESS).nonce(1).balance(1000);
        contract.chain.account(OWNER_ADDRESS).nonce(1).balance(1000);

        contract.deploy();

        contract
    }

    pub fn register_app(&mut self, burn_token: TokenIdentifier<StaticApi>) {
        self.chain
            .tx()
            .from(APP_ADDRESS)
            .to(STRATEGY_ADDRESS)
            .typed(strategy_burn_token_proxy::StrategyContractProxy)
            .register_app_endpoint(burn_token)
            .egld(1)
            .run();
    }

    fn deploy(&mut self) {
        self.chain
            .tx()
            .from(OWNER_ADDRESS)
            .typed(strategy_burn_token_proxy::StrategyContractProxy)
            .init()
            .code(CODE_PATH)
            .new_address(STRATEGY_ADDRESS)
            .run();
    }
}
