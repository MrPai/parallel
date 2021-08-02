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

/// Container for pending balance information
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, Default)]
pub struct UnstakeInfo<BlockNumber> {
    pub amount: Balance,
    pub block_number: BlockNumber,
    pub era_index: Option<EraIndex>,
}

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

        /// Currency type used for staking and liquid assets
        type Currency: MultiCurrencyExtended<
            Self::AccountId,
            CurrencyId = CurrencyId,
            Balance = Balance,
            Amount = Amount,
        >;

        /// Currency used for staking
        #[pallet::constant]
        type StakingCurrency: Get<CurrencyId>;

        /// Currency used for liquid voucher
        #[pallet::constant]
        type LiquidCurrency: Get<CurrencyId>;

        /// The pallet id of liquid staking, keeps all the staking assets.
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// The origin which can withdraw staking assets.
        type WithdrawOrigin: EnsureOrigin<Self::Origin>;

        /// The maximum assets can be withdrawed to a multisig account.
        #[pallet::constant]
        type MaxWithdrawAmount: Get<Balance>;

        /// The maximum size of AccountProcessingUnstake
        #[pallet::constant]
        type MaxAccountProcessingUnstake: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// XCM transfer
        type XcmTransfer: XcmTransfer<Self::AccountId, Balance, CurrencyId>;

        /// Approved agent list on relaychain
        type Members: SortedMembers<Self::AccountId>;

        /// Base xcm weight to use for cross chain transfer
        type BaseXcmWeight: Get<Weight>;

        type Bridge: RelaychainBridge;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// ExchangeRate is invalid
        InvalidExchangeRate,
        /// The withdraw assets exceed the threshold
        ExcessWithdrawThreshold,
        /// The account don't have any pending unstake
        NoPendingUnstake,
        /// The agent process invalid amount of unstake asset
        InvalidUnstakeAmount,
        /// There is no unstake in progress
        NoProcessingUnstake,
        /// There is no unstake in progress with input amount
        InvalidProcessedUnstakeAmount,
        /// The maximum account processing unstake reuqest exceeded
        MaxAccountProcessingUnstakeExceeded,
        /// Not approved agent
        IllegalAgent,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The assets get staked successfully
        Staked(T::AccountId, Balance),
        /// The voucher get unstaked successfully
        Unstaked(T::AccountId, Balance, Balance),
        /// The withdraw request is successful
        WithdrawSuccess(T::AccountId, Balance),
        /// The rewards are recorded
        RewardsRecorded(T::AccountId, Balance),
        /// The slash is recorded
        SlashRecorded(T::AccountId, Balance),
        /// The unstake request is processed
        UnstakeProcessed(T::AccountId, T::AccountId, Balance),
        /// The unstake reuqest is under processing by multisig account
        UnstakeProcessing(T::AccountId, T::AccountId, Balance),
    }

    /// The exchange rate converts staking native token to voucher.
    #[pallet::storage]
    #[pallet::getter(fn exchange_rate)]
    pub type ExchangeRate<T: Config> = StorageValue<_, Rate, ValueQuery>;

    /// Fraction of staking currency currently set aside for insurance pool
    #[pallet::storage]
    #[pallet::getter(fn reserve_factor)]
    pub type ReserveFactor<T: Config> = StorageValue<_, Ratio, ValueQuery>;

    /// The total amount of insurance pool.
    #[pallet::storage]
    #[pallet::getter(fn insurance_pool)]
    pub type InsurancePool<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// The total amount of staking pool.
    #[pallet::storage]
    #[pallet::getter(fn staking_pool)]
    pub type StakingPool<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// The queue stores all the pending unstaking requests.
    /// Key is the owner of assets.
    #[pallet::storage]
    #[pallet::getter(fn account_pending_unstake)]
    pub type AccountPendingUnstake<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, UnstakeInfo<T::BlockNumber>>;

    /// The queue stores all the unstaking requests in process.
    /// Key1 is the mutilsig agent in relaychain, key2 is the owner of assets.
    #[pallet::storage]
    #[pallet::getter(fn unstaking_processing_queue)]
    pub type AccountProcessingUnstake<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<UnstakeInfo<T::BlockNumber>, T::MaxAccountProcessingUnstake>,
    >;

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
        #[pallet::weight(10_000)]
        #[transactional]
        pub fn stake(
            origin: OriginFor<T>, 
            #[pallet::compact] amount: Balance
        ) -> DispatchResultWithPostInfo {
            <Self as LiquidStakingProtocol>::stake()?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        #[transactional]
        pub fn unstake(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            <Self as LiquidStakingProtocol>::unstake()?;
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        #[transactional]
        pub fn claim(
            origin: OriginFor<T>,
            #[pallet::compact] amount: Balance,
        ) -> DispatchResultWithPostInfo {
            <Self as LiquidStakingProtocol>::claim()?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }
}

impl<T: Config> ExchangeRateProvider for Pallet<T> {
    fn get_exchange_rate() -> Rate {
        ExchangeRate::<T>::get()
    }
}

impl<T: Config> LiquidStakingProtocol for Pallet<T> {
    // stake不需要为单个用户维护一个数据结构，因为用户可以立即获得xToken
    // 需要为当个era的stake总量维护一个total_pending_stake_amount,
    // 如果total_pending_stake_amount大于零, 那么当有用户unstake时，可以立即获得KSM

    // 如果total_pending_unstake_amount为零，每次有用户stake时，无法对冲，此时立即mint xtoken，多余的额度存储到total_pending_stake_amount中

    /// stake只做存储，不触发对冲，
    // 在每个era结束之时，检查total_pending_stake_amount或total_pending_unstake_amount是否有一方为空，如果不为空，使用差值进行操作
    // 差值留给下一次（也许在下一个era）用户unstake时去对冲

    fn stake() -> DispatchResultWithPostInfo {
        // 从bridge获取当前era index
        // 检查unstake队列，如果有xKSM的unstake请求，直接进行exchange，
        // 如果当个era内的xKSM被对冲到零，那么era结束时需要执行bond, 检查上个era是否执行了unbond，如果有，那么计算数量执行rebond
        T::Bridge::bond_extra(1,1);
        Ok(().into())
    }


    // unstake需要给每个用户增加一个存储，存储该用户的所有unstake请求，同时最多请求5次，每次记录era_index
    // 为总的unstake增加一个存储，total_pending_unstake_amount, 在有用户stake时做对冲（减少数量），或者在该era结束时执行unbond 
    // 

    //如果total_pending_stake_amount为零，每次有用户unstake时，无法对冲，此时存储数据到单个用户的map中，同时增加total_pending_unstake_amount；

    //////////// 问题的关键 //////////// 
    // 如果此时有用户stake了，来对total_pending_unstake_amount进行对冲时，如何给每个用户分发ksm呢？
    // 可以按照数量大小进行升序，unstake的数量越小，越有可能先立即获得KSM； （这应该也鼓励用户每次质押的尽可能少）

    // 尝试使用topK算法解决这个问题
    // 简单的topK算法不能解决，因为需求是找到K个元素，他们的value最小且value之和基本和total_pending_stake_amount相当。
    // 前期考虑使用迭代map的方式实现，后期在算法上拓展。
    //////////////////////////////////// 


    // 当stake操作对total_pending_unstake_amount进行对冲时，除了总量因对冲减少外，还需要将对冲的部分分摊到指定的用户上，
    // 分摊的部分不会进行转账，而是存储到一个era_index小于current_index的存储中（或者单独一个Withdrawable amount的存储），
    // 然后由用户调用claim，进行实际的提取操作（根据可提取额度进行转账）


    /// 每次有用户unstake时，检查total_pending_stake_amount是否为空，如果不为空，则对冲
    fn unstake() -> DispatchResultWithPostInfo {
        // 从bridge获取当前era index
        // 
        // 
        T::Bridge::unbond(1,1);
        Ok(().into())
    }

    fn claim() -> DispatchResultWithPostInfo {
        // 从bridge获取当前era index
        // 比较unstake时刻的claim与当前的era index
        Ok(().into())
    }
}
