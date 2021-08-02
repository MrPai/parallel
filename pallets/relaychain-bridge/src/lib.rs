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
use primitives::liquid_staking::*;

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

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub exchange_rate: Rate,
        pub reserve_factor: Ratio,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                exchange_rate: Rate::default(),
                reserve_factor: Ratio::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            ExchangeRate::<T>::put(self.exchange_rate);
            ReserveFactor::<T>::put(self.reserve_factor);
        }
    }

    #[cfg(feature = "std")]
    impl GenesisConfig {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage<T: Config>(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }

        /// Direct implementation of `GenesisBuild::assimilate_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn assimilate_storage<T: Config>(
            &self,
            storage: &mut sp_runtime::Storage,
        ) -> Result<(), String> {
            <Self as GenesisBuild<T>>::assimilate_storage(self, storage)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 这些主要由stake-client来调用
        #[pallet::weight(10_000)]
        #[transactional]
        pub fn x1(
            origin: OriginFor<T>, 
            #[pallet::compact] amount: Balance
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        #[transactional]
        pub fn x2(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        #[transactional]
        pub fn x3(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            Ok(().into())
        }
    }
}


impl<T: Config> RelaychainBridge for Pallet<T> {
    fn bond_extra(account_index: u32, amount: Balance) -> DispatchResult;
	fn unbond(account_index: u32, amount: Balance) -> DispatchResult;
	fn rebond(account_index: u32, amount: Balance) -> DispatchResult;
	fn withdraw_unbonded(account_index: u32);
	fn nominate(account_index: u32, targets: Vec<Self::PolkadotAccountId>);
	fn transfer_to_relaychain(account_index: u32, from: &AccountId, amount: Balance) -> DispatchResult;
	fn receive_from_relaychain(account_index: u32, to: &AccountId, amount: Balance) -> DispatchResult;
	fn payout_stakers(account_index: u32, era: EraIndex);
}