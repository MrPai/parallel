use super::Balance;
pub type EraIndex = u32;

//修改返回参数，因为如果amm调用需要的是一个返回值
pub trait LiquidStakingProtocol {
    fn stake() -> DispatchResultWithPostInfo;
    fn unstake() -> DispatchResultWithPostInfo;
    fn claim() -> DispatchResultWithPostInfo;
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum PalletType {
	LiquidStaking(LiquidStakingMethod),
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum LiquidStakingMethod {
	RecordReward(Balance),
	RecordSlash(Balance),
	TriggerNewEra(EraIndex),
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum Phase {
	Started,
	UpdateEraIndex,
	RecordReward,
	DispatchToStaking,
	RecordStakingOperation,
	Finished,
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum StakingOperationType {
	Bond,
	BondExtra,
	Unbond,
	Rebond,
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum StakingOperationStatus {
	Ready,
	Processing,
	Successed,
	Failed,
}

pub trait LiquidStakingHub {
    fn request_stake() -> DispatchResultWithPostInfo;
    fn request_unstake() -> DispatchResultWithPostInfo;
	// todo should be current era + 1, but if not, warning.
    fn trigger_new_era(era_index: EraIndex) -> DispatchResult;
    fn record_reward() -> DispatchResultWithPostInfo;
    fn record_slash() -> DispatchResultWithPostInfo;
	// query liquidStaking pool status and decide whether to bond_extra/unbond/rebond
    fn request_to_relaychain() -> StakingOperationType;
	fn response_from_relaychain() -> StakingOperationType;
	
}


// 在liquid-staking pallet新建一个config type，可以调用bridge的方法
// stake-client将直接与bridge通讯
pub trait RelaychainBridgeHub<AccountId, BlockNumber, Balance, EraIndex>
{
    type BondingDuration: Get<EraIndex>;
	type EraLength: Get<BlockNumber>;
	type PolkadotAccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default;
	
	fn request_to_relaychain() -> DispatchResultWithPostInfo;
	fn response_from_relaychain() -> DispatchResultWithPostInfo;
	
	//调用此接口，stake client获取到具体应该调用relaychain的事件类型和相关信息
	// fn emit_to_relaychain() -> DispatchResultWithPostInfo;
	
	// fn bond(account_index: u32, amount: Balance) -> DispatchResult;
	// fn bond_extra(account_index: u32, amount: Balance) -> DispatchResult;
	// fn unbond(account_index: u32, amount: Balance) -> DispatchResult;
	// fn rebond(account_index: u32, amount: Balance) -> DispatchResult;
	// fn withdraw_unbonded(account_index: u32);
	// fn nominate(account_index: u32, targets: Vec<Self::PolkadotAccountId>);
	// fn transfer_to_relaychain(account_index: u32, from: &AccountId, amount: Balance) -> DispatchResult;
	// fn receive_from_relaychain(account_index: u32, to: &AccountId, amount: Balance) -> DispatchResult;
	// fn payout_stakers(account_index: u32, era: EraIndex);
}