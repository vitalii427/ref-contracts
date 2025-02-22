use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance, PromiseOrValue};

use crate::admin_fee::AdminFees;
use crate::simple_pool::SimplePool;
use crate::stable_swap::StableSwapPool;
use crate::rated_swap::RatedSwapPool;
use crate::rated_swap::rates::RatesTrait;
use crate::utils::SwapVolume;

/// Generic Pool, providing wrapper around different implementations of swap pools.
/// Allows to add new types of pools just by adding extra item in the enum without needing to migrate the storage.
#[derive(BorshSerialize, BorshDeserialize)]
pub enum Pool {
    SimplePool(SimplePool),
    StableSwapPool(StableSwapPool),
    RatedSwapPool(RatedSwapPool),
}

impl Pool {
    /// Returns pool kind.
    pub fn kind(&self) -> String {
        match self {
            Pool::SimplePool(_) => "SIMPLE_POOL".to_string(),
            Pool::StableSwapPool(_) => "STABLE_SWAP".to_string(),
            Pool::RatedSwapPool(_) => "RATED_SWAP".to_string(),
        }
    }

    /// Returns which tokens are in the underlying pool.
    pub fn tokens(&self) -> &[AccountId] {
        match self {
            Pool::SimplePool(pool) => pool.tokens(),
            Pool::StableSwapPool(pool) => pool.tokens(),
            Pool::RatedSwapPool(pool) => pool.tokens(),
        }
    }

    /// Adds liquidity into underlying pool.
    /// Updates amounts to amount kept in the pool.
    pub fn add_liquidity(
        &mut self,
        sender_id: &AccountId,
        amounts: &mut Vec<Balance>,
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.add_liquidity(sender_id, amounts),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(_) => unimplemented!(),
        }
    }

    pub fn add_stable_liquidity(
        &mut self,
        sender_id: &AccountId,
        amounts: &Vec<Balance>,
        min_shares: Balance,
        admin_fee: AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.add_liquidity(sender_id, amounts, min_shares, &admin_fee),
            Pool::RatedSwapPool(pool) => pool.add_liquidity(sender_id, amounts, min_shares, &admin_fee),
        }
    }

    /// Removes liquidity from underlying pool.
    pub fn remove_liquidity(
        &mut self,
        sender_id: &AccountId,
        shares: Balance,
        min_amounts: Vec<Balance>,
    ) -> Vec<Balance> {
        match self {
            Pool::SimplePool(pool) => pool.remove_liquidity(sender_id, shares, min_amounts),
            Pool::StableSwapPool(pool) => {
                pool.remove_liquidity_by_shares(sender_id, shares, min_amounts)
            },
            Pool::RatedSwapPool(pool) => {
                pool.remove_liquidity_by_shares(sender_id, shares, min_amounts)
            }
        }
    }

    /// Removes liquidity from underlying pool.
    pub fn remove_liquidity_by_tokens(
        &mut self,
        sender_id: &AccountId,
        amounts: Vec<Balance>,
        max_burn_shares: Balance,
        admin_fee: AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => {
                pool.remove_liquidity_by_tokens(sender_id, amounts, max_burn_shares, &admin_fee)
            },
            Pool::RatedSwapPool(pool) => {
                pool.remove_liquidity_by_tokens(sender_id, amounts, max_burn_shares, &admin_fee)
            }
        }
    }

    /// Returns how many tokens will one receive swapping given amount of token_in for token_out.
    pub fn get_return(
        &self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.get_return(token_in, amount_in, token_out),
            Pool::StableSwapPool(pool) => pool.get_return(token_in, amount_in, token_out, fees),
            Pool::RatedSwapPool(pool) => pool.get_return(token_in, amount_in, token_out, fees),
        }
    }

    /// Return share decimal.
    pub fn get_share_decimal(&self) -> u8 {
        match self {
            Pool::SimplePool(_) => 24,
            Pool::StableSwapPool(_) => 18,
            Pool::RatedSwapPool(_) => 24,
        }
    }

    /// Returns given pool's total fee.
    pub fn get_fee(&self) -> u32 {
        match self {
            Pool::SimplePool(pool) => pool.get_fee(),
            Pool::StableSwapPool(pool) => pool.get_fee(),
            Pool::RatedSwapPool(pool) => pool.get_fee(),
        }
    }

    /// Returns volumes of the given pool.
    pub fn get_volumes(&self) -> Vec<SwapVolume> {
        match self {
            Pool::SimplePool(pool) => pool.get_volumes(),
            Pool::StableSwapPool(pool) => pool.get_volumes(),
            Pool::RatedSwapPool(pool) => pool.get_volumes(),
        }
    }

    /// Returns given pool's share price in precision 1e8.
    pub fn get_share_price(&self) -> u128 {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.get_share_price(),
            Pool::RatedSwapPool(pool) => pool.get_share_price(),
        }
    }

    /// Swaps given number of token_in for token_out and returns received amount.
    pub fn swap(
        &mut self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        min_amount_out: Balance,
        admin_fee: AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee)
            }
            Pool::StableSwapPool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee)
            }
            Pool::RatedSwapPool(pool) => {
                pool.swap(token_in, amount_in, token_out, min_amount_out, &admin_fee)
            }
        }
    }

    pub fn share_total_balance(&self) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.share_total_balance(),
            Pool::StableSwapPool(pool) => pool.share_total_balance(),
            Pool::RatedSwapPool(pool) => pool.share_total_balance(),
        }
    }

    pub fn share_balances(&self, account_id: &AccountId) -> Balance {
        match self {
            Pool::SimplePool(pool) => pool.share_balance_of(account_id),
            Pool::StableSwapPool(pool) => pool.share_balance_of(account_id),
            Pool::RatedSwapPool(pool) => pool.share_balance_of(account_id),
        }
    }

    pub fn share_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: u128) {
        match self {
            Pool::SimplePool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
            Pool::StableSwapPool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
            Pool::RatedSwapPool(pool) => pool.share_transfer(sender_id, receiver_id, amount),
        }
    }

    pub fn share_register(&mut self, account_id: &AccountId) {
        match self {
            Pool::SimplePool(pool) => pool.share_register(account_id),
            Pool::StableSwapPool(pool) => pool.share_register(account_id),
            Pool::RatedSwapPool(pool) => pool.share_register(account_id),
        }
    }

    pub fn predict_add_stable_liquidity(
        &self,
        amounts: &Vec<Balance>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.predict_add_stable_liquidity(amounts, fees),
            Pool::RatedSwapPool(_) => unimplemented!(),
        }
    }

    pub fn predict_remove_liquidity(
        &self,
        shares: Balance,
    ) -> Vec<Balance> {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.predict_remove_liquidity(shares),
            Pool::RatedSwapPool(pool) => pool.predict_remove_liquidity(shares),
        }
    }

    pub fn predict_remove_liquidity_by_tokens(
        &self,
        amounts: &Vec<Balance>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(pool) => pool.predict_remove_liquidity_by_tokens(amounts, fees),
            Pool::RatedSwapPool(pool) => pool.predict_remove_liquidity_by_tokens(amounts, fees),
        }
    }

    pub fn predict_add_rated_liquidity(
        &self,
        amounts: &Vec<Balance>,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.predict_add_rated_liquidity(amounts, rates, fees),
        }
    }

    pub fn predict_remove_rated_liquidity_by_tokens(
        &self,
        amounts: &Vec<Balance>,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.predict_remove_rated_liquidity_by_tokens(amounts, rates, fees),
        }
    }

    pub fn get_rated_return(
        &self,
        token_in: &AccountId,
        amount_in: Balance,
        token_out: &AccountId,
        rates: &Option<Vec<Balance>>,
        fees: &AdminFees,
    ) -> Balance {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.get_rated_return(token_in, amount_in, token_out, rates, fees),
        }
    }

    pub fn update_rates(&self) -> PromiseOrValue<bool> {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.rates.update(),
        }
    }

    pub fn update_callback(&mut self, cross_call_result: &Vec<u8>) -> bool {
        match self {
            Pool::SimplePool(_) => unimplemented!(),
            Pool::StableSwapPool(_) => unimplemented!(),
            Pool::RatedSwapPool(pool) => pool.rates.update_callback(cross_call_result),
        }
    }
}
