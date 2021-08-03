// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Liquid staking pallet
//!
//! ## Overview
//!
//! This pallet manages the NPoS operations for relay chain asset.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*, traits::SortedMembers, transactional, BoundedVec, PalletId,
};
use frame_system::pallet_prelude::*;
use orml_traits::XcmTransfer;
use sp_runtime::{traits::AccountIdConversion, ArithmeticError, FixedPointNumber, RuntimeDebug};
use sp_std::convert::TryInto;
use sp_std::prelude::*;
use xcm::v0::{Junction, MultiLocation, NetworkId};

use orml_traits::{MultiCurrency, MultiCurrencyExtended};

pub use pallet::*;
use primitives::{Amount, Balance, CurrencyId, ExchangeRateProvider, Rate, Ratio};
use primitives::liquid_staking::{EraIndex,LiquidStakingHub, StakingOperationType, StakingOperationStatus,RelaychainBridgeHub,Phase};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type LiquidStakingHub: RelaychainBridgeHub;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// ExchangeRate is invalid
        InvalidExchangeRate,
        /// The withdraw assets exceed the threshold
        ExcessWithdrawThreshold,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The assets get staked successfully
        Staked(T::AccountId, Balance),
        /// The voucher get unstaked successfully
        Unstaked(T::AccountId, Balance, Balance),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[transactional]
        pub fn request_to_relaychain(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            // todo 将结果返回给调用方，可以尝试将本函数的返回结果是一串unsign的call 调用， stake-client拿到后可以直接签名并发送到relaychain上
            Self::request_to_relaychain()?;
            
            Ok(().into())
        }

        // todo start from here， 尝试将record_reward/record_slash/trigger_new_era三个方法整合到下述这个方法里
        // todo 直接使用call作为参数类型（但是call貌似必须是交易类型的方法，而不是trait的）
        #[pallet::weight(10_000)]
        #[transactional]
        pub fn response_from_relaychain(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            Self::response_from_relaychain()?;
            Ok(().into())
        }

        // #[pallet::weight(10_000)]
        // #[transactional]
        // pub fn reconciliation(
        //     origin: OriginFor<T>,
        //     #[pallet::compact] amount: Balance,
        // ) -> DispatchResultWithPostInfo {
        //     // 考虑是否增加这样一个对账接口，对比中继链与平行链的账本
        //     ensure!(Self::current_phase() == Phase::RecordStakingOperation,"big error");

        //     CurrentPhase::<T>::put(Phase::Started);
        // }
    }
}


impl<T: Config> RelaychainBridgeHub for Pallet<T> {
    fn request_to_relaychain() -> DispatchResultWithPostInfo{
        // query liquidStaking pool status and decide whether to bond_extra/unbond/rebond
        T::LiquidStakingHub::request_to_relaychain();
        Ok(().into())
    }
    //pallet type
    //method type
    //argument list
	fn response_from_relaychain() -> DispatchResultWithPostInfo{
        T::LiquidStakingHub::response_from_relaychain();


        

        Ok(().into())
    }

    
	// fn bond(account_index: u32, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn bond_extra(account_index: u32, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn unbond(account_index: u32, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn rebond(account_index: u32, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn withdraw_unbonded(account_index: u32){
    //     Ok(().into())
    // }
	// fn nominate(account_index: u32, targets: Vec<Self::PolkadotAccountId>){
    //     Ok(().into())
    // }
	// fn transfer_to_relaychain(account_index: u32, from: &AccountId, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn receive_from_relaychain(account_index: u32, to: &AccountId, amount: Balance) -> DispatchResult{
    //     Ok(().into())
    // }
	// fn payout_stakers(account_index: u32, era: EraIndex){
    //     Ok(().into())
    // }
}