use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use super::Balance;

pub type EraIndex = u32;

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum StakingOperationType {
	Bond,
	BondExtra,
	Unbond,
	Rebond,
	TransferToRelaychain,
	RecordReward,
	RecordSlash,
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum LiquidStakingHubMethod {
	// Hedging,
	EmitEventToRelaychain,
	TransferToRelaychain(Balance),
	TriggerNewEra(EraIndex),
	RecordReward(Balance),
	RecordSlash(Balance),
    RecordBondResponse,
    RecordBondExtraResponse,
    RecordUnbondResponse,
    RecordRebondResponse,
	RecordXcmTransfer,
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum Phase {
	Started,
	UpdateEraIndex,
	RecordReward,
	EmitEventToRelaychain,
	RecordStakingOperation,
	Finished,
}

//修改返回参数，因为如果amm调用需要的是一个返回值
pub trait LiquidStakingProtocol {
    fn stake() -> DispatchResultWithPostInfo;
    fn unstake() -> DispatchResultWithPostInfo;
    fn claim() -> DispatchResultWithPostInfo;
}

pub trait LiquidStakingHub {
    fn request_stake() -> DispatchResultWithPostInfo;
    fn request_unstake() -> DispatchResultWithPostInfo;

    // fn hedging() -> DispatchResultWithPostInfo;

	fn transfer_to_relaychain() -> DispatchResultWithPostInfo;
	fn emit_event_to_relaychain() -> DispatchResultWithPostInfo;

	// todo should be current era + 1, but if not, warning.
    fn trigger_new_era(era_index: EraIndex) -> DispatchResultWithPostInfo;
    fn record_reward() -> DispatchResultWithPostInfo;
    fn record_slash() -> DispatchResultWithPostInfo;
    fn record_bond_response() -> DispatchResultWithPostInfo;
    fn record_bond_extra_response() -> DispatchResultWithPostInfo;
    fn record_unbond_response() -> DispatchResultWithPostInfo;
    fn record_rebond_response() -> DispatchResultWithPostInfo;
}