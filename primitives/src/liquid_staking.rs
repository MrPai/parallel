pub type EraIndex = u32;

//修改返回参数，因为如果amm调用需要的是一个返回值
pub trait LiquidStakingProtocol {
    fn stake() -> DispatchResultWithPostInfo;
    fn unstake() -> DispatchResultWithPostInfo;
    fn claim() -> DispatchResultWithPostInfo;
}

// 在liquid-staking pallet新建一个config type，可以调用bridge的方法
// stake-client将直接与bridge通讯
pub trait RelaychainBridge<AccountId, BlockNumber, Balance, EraIndex>
{
    type BondingDuration: Get<EraIndex>;
	type EraLength: Get<BlockNumber>;
	type PolkadotAccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default;
    
	fn bond_extra(account_index: u32, amount: Balance) -> DispatchResult;
	fn unbond(account_index: u32, amount: Balance) -> DispatchResult;
	fn rebond(account_index: u32, amount: Balance) -> DispatchResult;
	fn withdraw_unbonded(account_index: u32);
	fn nominate(account_index: u32, targets: Vec<Self::PolkadotAccountId>);
	fn transfer_to_relaychain(account_index: u32, from: &AccountId, amount: Balance) -> DispatchResult;
	fn receive_from_relaychain(account_index: u32, to: &AccountId, amount: Balance) -> DispatchResult;
	fn payout_stakers(account_index: u32, era: EraIndex);
}