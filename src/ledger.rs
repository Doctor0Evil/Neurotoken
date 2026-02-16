use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::accounts::{Account, AccountId};
use crate::error::{NeuroError, NeuroResult};
use crate::token::{TokenMeta, TokenSupply};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: u128,
    pub symbol: String,
}

impl TransferRequest {
    pub fn new<S: Into<String>>(
        from: AccountId,
        to: AccountId,
        amount: u128,
        symbol: S,
    ) -> Self {
        Self {
            from,
            to,
            amount,
            symbol: symbol.into(),
        }
    }
}

/// In-memory ledger; single-threaded by design (no parking_lot, no RwLock).
#[derive(Debug, Default)]
pub struct Ledger {
    accounts: HashMap<AccountId, Account>,
    aliases: HashMap<String, AccountId>,
    supplies: HashMap<String, TokenSupply>,
    tokens: HashMap<String, TokenMeta>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_account<S: Into<String>>(&mut self, alias: S) -> NeuroResult<AccountId> {
        let alias_str = alias.into();
        if self.aliases.contains_key(&alias_str) {
            return Err(NeuroError::DuplicateAccountAlias(alias_str));
        }

        let account = Account::new(&alias_str);
        let id = account.id.clone();
        self.aliases.insert(alias_str, id.clone());
        self.accounts.insert(id.clone(), account);
        Ok(id)
    }

    pub fn get_account(&self, id: &AccountId) -> Option<&Account> {
        self.accounts.get(id)
    }

    pub fn register_token(&mut self, meta: TokenMeta) {
        self.tokens.entry(meta.symbol.clone()).or_insert(meta);
        self.supplies
            .entry(meta.symbol.clone())
            .or_insert_with(TokenSupply::new);
    }

    pub fn mint(
        &mut self,
        account_id: &AccountId,
        amount: u128,
        meta: TokenMeta,
    ) -> NeuroResult<()> {
        if amount == 0 {
            return Err(NeuroError::InvalidAmount(amount));
        }

        if !self.tokens.contains_key(&meta.symbol) {
            self.register_token(meta);
        }

        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or_else(|| NeuroError::AccountNotFound(account_id.clone()))?;

        let supply = self
            .supplies
            .get_mut(&meta.symbol)
            .ok_or_else(|| NeuroError::TokenNotFound(meta.symbol.clone()))?;

        supply.mint(amount).ok_or(NeuroError::Overflow)?;
        account.credit(&meta.symbol, amount)?;
        Ok(())
    }

    pub fn transfer(&mut self, req: TransferRequest) -> NeuroResult<()> {
        if req.amount == 0 {
            return Err(NeuroError::InvalidAmount(req.amount));
        }

        let from = self
            .accounts
            .get_mut(&req.from)
            .ok_or_else(|| NeuroError::AccountNotFound(req.from.clone()))?;
        let to = self
            .accounts
            .get_mut(&req.to)
            .ok_or_else(|| NeuroError::AccountNotFound(req.to.clone()))?;

        from.debit(&req.symbol, req.amount)?;
        to.credit(&req.symbol, req.amount)?;
        Ok(())
    }

    pub fn burn(
        &mut self,
        account_id: &AccountId,
        symbol: &str,
        amount: u128,
    ) -> NeuroResult<()> {
        if amount == 0 {
            return Err(NeuroError::InvalidAmount(amount));
        }

        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or_else(|| NeuroError::AccountNotFound(account_id.clone()))?;

        account.debit(symbol, amount)?;

        let supply = self
            .supplies
            .get_mut(symbol)
            .ok_or_else(|| NeuroError::TokenNotFound(symbol.to_owned()))?;

        supply.burn(amount).ok_or(NeuroError::Overflow)?;
        Ok(())
    }

    pub fn balance_of(&self, account_id: &AccountId, symbol: &str) -> Option<u128> {
        self.accounts
            .get(account_id)
            .and_then(|acc| acc.get_balance(symbol))
    }

    pub fn total_supply(&self, symbol: &str) -> Option<u128> {
        self.supplies.get(symbol).map(|s| s.total)
    }
}
