
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use crate::liquid_staking::LiquidStakingHubMethod;

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum ParachainPallet {
	LiquidStaking(LiquidStakingHubMethod),
}

#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum ResponseStatus {
	Ready,
	Processing,
	Successed,
	Failed,
}


// 在liquid-staking pallet新建一个config type，可以调用bridge的方法
// stake-client将直接与bridge通讯
pub trait RelaychainBridgeHub<AccountId>{
    // type BondingDuration: Get<EraIndex>;
	// type EraLength: Get<BlockNumber>;
	// type PolkadotAccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default;
	
	//调用此接口，stake client获取到具体应该调用relaychain的事件类型和相关信息
	fn request_to_relaychain(who: &AccountId, parachain_pallet: &ParachainPallet) -> DispatchResultWithPostInfo;
	fn response_from_relaychain(who: &AccountId, parachain_pallet: &ParachainPallet, response_status: &ResponseStatus) -> DispatchResultWithPostInfo;
}