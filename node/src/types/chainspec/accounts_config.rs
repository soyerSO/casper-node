use std::path::Path;

use datasize::DataSize;
use num::Zero;
#[cfg(test)]
use rand::{distributions::Standard, prelude::*};
use serde::{Deserialize, Serialize};

use casper_execution_engine::{core::engine_state::GenesisAccount, shared::motes::Motes};
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    PublicKey,
};
#[cfg(test)]
use casper_types::{SecretKey, U512};

#[cfg(test)]
use crate::testing::TestRng;
use crate::utils::{self, Loadable};

use super::error::ChainspecAccountsLoadError;

const CHAINSPEC_ACCOUNTS_FILENAME: &str = "accounts.toml";

#[derive(PartialEq, Eq, Serialize, Deserialize, DataSize, Debug, Copy, Clone)]
pub struct AccountConfig {
    public_key: PublicKey,
    balance: Motes,
    bonded_amount: Motes,
}

impl AccountConfig {
    pub fn new(public_key: PublicKey, balance: Motes, bonded_amount: Motes) -> Self {
        Self {
            public_key,
            balance,
            bonded_amount,
        }
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn balance(&self) -> Motes {
        self.balance
    }

    pub fn bonded_amount(&self) -> Motes {
        self.bonded_amount
    }

    pub fn is_genesis_validator(&self) -> bool {
        !self.bonded_amount.is_zero()
    }

    #[cfg(test)]
    /// Generates a random instance using a `TestRng`.
    pub fn random(rng: &mut TestRng) -> Self {
        let public_key = PublicKey::from(&SecretKey::ed25519(rng.gen()));
        let balance = Motes::new(U512::from(rng.gen::<u64>()));
        let bonded_amount = Motes::new(U512::from(rng.gen::<u64>()));

        AccountConfig {
            public_key,
            balance,
            bonded_amount,
        }
    }
}

#[cfg(test)]
impl Distribution<AccountConfig> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AccountConfig {
        let public_key = SecretKey::ed25519(rng.gen()).into();

        let mut u512_array = [0u8; 64];
        rng.fill_bytes(u512_array.as_mut());
        let balance = Motes::new(U512::from(u512_array));

        rng.fill_bytes(u512_array.as_mut());
        let bonded_amount = Motes::new(U512::from(u512_array));

        AccountConfig::new(public_key, balance, bonded_amount)
    }
}

impl ToBytes for AccountConfig {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.public_key.to_bytes()?);
        buffer.extend(self.balance.to_bytes()?);
        buffer.extend(self.bonded_amount.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.public_key.serialized_length()
            + self.balance.serialized_length()
            + self.bonded_amount.serialized_length()
    }
}

impl FromBytes for AccountConfig {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (public_key, remainder) = FromBytes::from_bytes(bytes)?;
        let (balance, remainder) = FromBytes::from_bytes(remainder)?;
        let (bonded_amount, remainder) = FromBytes::from_bytes(remainder)?;
        let account_config = AccountConfig {
            public_key,
            balance,
            bonded_amount,
        };
        Ok((account_config, remainder))
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, DataSize, Debug, Copy, Clone)]
pub struct DelegatorConfig {
    validator_public_key: PublicKey,
    delegator_public_key: PublicKey,
    balance: Motes,
    delegated_amount: Motes,
}

impl DelegatorConfig {
    pub fn new(
        validator_public_key: PublicKey,
        delegator_public_key: PublicKey,
        balance: Motes,
        delegated_amount: Motes,
    ) -> Self {
        Self {
            validator_public_key,
            delegator_public_key,
            balance,
            delegated_amount,
        }
    }

    pub fn validator_public_key(&self) -> PublicKey {
        self.validator_public_key
    }

    pub fn delegator_public_key(&self) -> PublicKey {
        self.delegator_public_key
    }

    pub fn balance(&self) -> Motes {
        self.balance
    }

    pub fn delegated_amount(&self) -> Motes {
        self.delegated_amount
    }

    #[cfg(test)]
    /// Generates a random instance using a `TestRng`.
    pub fn random(rng: &mut TestRng) -> Self {
        let validator_public_key = PublicKey::from(&SecretKey::ed25519(rng.gen()));
        let delegator_public_key = PublicKey::from(&SecretKey::ed25519(rng.gen()));
        let balance = Motes::new(U512::from(rng.gen::<u64>()));
        let delegated_amount = Motes::new(U512::from(rng.gen::<u64>()));

        DelegatorConfig {
            validator_public_key,
            delegator_public_key,
            balance,
            delegated_amount,
        }
    }
}

#[cfg(test)]
impl Distribution<DelegatorConfig> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DelegatorConfig {
        let validator_public_key = SecretKey::ed25519(rng.gen()).into();
        let delegator_public_key = SecretKey::ed25519(rng.gen()).into();

        let mut u512_array = [0u8; 64];
        rng.fill_bytes(u512_array.as_mut());
        let balance = Motes::new(U512::from(u512_array));

        rng.fill_bytes(u512_array.as_mut());
        let delegated_amount = Motes::new(U512::from(u512_array));

        DelegatorConfig::new(
            validator_public_key,
            delegator_public_key,
            balance,
            delegated_amount,
        )
    }
}

impl ToBytes for DelegatorConfig {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.validator_public_key.to_bytes()?);
        buffer.extend(self.delegator_public_key.to_bytes()?);
        buffer.extend(self.balance.to_bytes()?);
        buffer.extend(self.delegated_amount.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.validator_public_key.serialized_length()
            + self.delegator_public_key.serialized_length()
            + self.balance.serialized_length()
            + self.delegated_amount.serialized_length()
    }
}

impl FromBytes for DelegatorConfig {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (validator_public_key, remainder) = FromBytes::from_bytes(bytes)?;
        let (delegator_public_key, remainder) = FromBytes::from_bytes(remainder)?;
        let (balance, remainder) = FromBytes::from_bytes(remainder)?;
        let (delegated_amount, remainder) = FromBytes::from_bytes(remainder)?;
        let delegator_config = DelegatorConfig {
            validator_public_key,
            delegator_public_key,
            balance,
            delegated_amount,
        };
        Ok((delegator_config, remainder))
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, DataSize, Debug, Clone)]
pub struct AccountsConfig {
    accounts: Vec<AccountConfig>,
    #[serde(default)]
    delegators: Vec<DelegatorConfig>,
}

impl AccountsConfig {
    pub fn new(accounts: Vec<AccountConfig>, delegators: Vec<DelegatorConfig>) -> Self {
        Self {
            accounts,
            delegators,
        }
    }

    pub fn accounts(&self) -> &[AccountConfig] {
        &self.accounts
    }

    pub fn delegators(&self) -> &[DelegatorConfig] {
        &self.delegators
    }

    #[cfg(test)]
    /// Generates a random instance using a `TestRng`.
    pub fn random(rng: &mut TestRng) -> Self {
        let alpha = AccountConfig::random(rng);
        let accounts = vec![
            alpha,
            AccountConfig::random(rng),
            AccountConfig::random(rng),
            AccountConfig::random(rng),
        ];

        let mut delegator = DelegatorConfig::random(rng);
        delegator.validator_public_key = alpha.public_key;

        let delegators = vec![delegator];

        AccountsConfig {
            accounts,
            delegators,
        }
    }
}

impl ToBytes for AccountsConfig {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.accounts.to_bytes()?);
        buffer.extend(self.delegators.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.accounts.serialized_length() + self.delegators.serialized_length()
    }
}

impl FromBytes for AccountsConfig {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (accounts, remainder) = FromBytes::from_bytes(bytes)?;
        let (delegators, remainder) = FromBytes::from_bytes(remainder)?;
        let accounts_config = AccountsConfig::new(accounts, delegators);
        Ok((accounts_config, remainder))
    }
}

impl Loadable for AccountsConfig {
    type Error = ChainspecAccountsLoadError;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let accounts_path = path.as_ref().join(CHAINSPEC_ACCOUNTS_FILENAME);
        if !accounts_path.is_file() {
            return Ok(AccountsConfig::new(vec![], vec![]));
        }
        let bytes = utils::read_file(accounts_path)?;
        let toml_chainspec: AccountsConfig = toml::from_slice(&bytes)?;
        Ok(toml_chainspec)
    }
}

impl From<AccountsConfig> for Vec<GenesisAccount> {
    fn from(accounts_config: AccountsConfig) -> Self {
        let mut genesis_accounts = Vec::with_capacity(accounts_config.accounts.len());
        for account in accounts_config.accounts {
            let genesis_account = GenesisAccount::new(
                account.public_key,
                account.public_key.to_account_hash(),
                account.balance,
                account.bonded_amount,
            );
            genesis_accounts.push(genesis_account);
        }
        genesis_accounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_roundtrip() {
        let mut rng = TestRng::new();
        let accounts_config = AccountsConfig::random(&mut rng);
        bytesrepr::test_serialization_roundtrip(&accounts_config);
    }
}
