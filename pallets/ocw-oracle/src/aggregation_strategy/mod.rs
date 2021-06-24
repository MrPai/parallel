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

use crate::*;

mod median;
use self::median::*;

mod average;
use self::average::*;
pub trait AggregationStrategyApi<T: Config> {
    fn aggregate_price(
        round_index: &RoundIndex<T::BlockNumber>,
        provider: &Vec<T::AccountId>,
        currency_id: &CurrencyId,
    ) -> Result<PriceDetail, Error<T>>;
}

pub fn aggregate_price<T: Config>(
    aggregate_strategy: AggregationStrategyEnum,
    round_index: &RoundIndex<T::BlockNumber>,
    provider: &Vec<T::AccountId>,
    currency_id: &CurrencyId,
) -> Result<PriceDetail, Error<T>> {
    match aggregate_strategy {
        AggregationStrategyEnum::EMERGENCY => Err(<Error<T>>::NotImplement.into()),
        AggregationStrategyEnum::MEDIAN => {
            Median::aggregate_price(round_index, provider, currency_id)
        }
        AggregationStrategyEnum::AVERAGE => {
            Average::aggregate_price(round_index, provider, currency_id)
        }
    }
}
