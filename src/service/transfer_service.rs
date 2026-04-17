use crate::domain::Address;
use crate::domain::Balance;
use crate::domain::Wallet;
use crate::repository::db_provider::TransactionHandler;
use crate::repository::{TransactionProvider, WalletRepository};
use anyhow::{Result, anyhow};

pub struct TransferService<R, P>
where
    R: WalletRepository,
    P: TransactionProvider,
{
    repo: R,
    provider: P,
}

impl<R, P> TransferService<R, P>
where
    R: WalletRepository<Tx = P::Tx>,
    P: TransactionProvider,
    // Don't care what DB is used so long as the tx it creates knows how to commit itself
    P::Tx: TransactionHandler,
{
    pub fn new(repo: R, provider: P) -> Self {
        Self { repo, provider }
    }

    /// Encapsulates the deterministic locking and role mapping logic.
    /// Returns (Sender, Receiver)
    async fn lock_and_fetch_pair(
        &self,
        tx: &mut P::Tx, // We use the associated type for the transaction
        from_address: &Address,
        to_address: &Address,
    ) -> Result<(Wallet, Wallet)> {
        // 1. DETERMINISTIC ORDERING (Prevents Deadlocks)
        let (first_addr, second_addr) = if from_address.as_str() < to_address.as_str() {
            (from_address, to_address)
        } else {
            (to_address, from_address)
        };

        // 2. PESSIMISTIC LOCKING
        let first_wallet = self
            .repo
            .find_by_address_for_update(tx, first_addr)
            .await?
            .ok_or_else(|| anyhow!("Wallet not found: {}", first_addr))?;

        let second_wallet = self
            .repo
            .find_by_address_for_update(tx, second_addr)
            .await?
            .ok_or_else(|| anyhow!("Wallet not found: {}", second_addr))?;

        // 3. ROLE MAPPING (Identifies who is who)
        if first_wallet.address() == from_address.as_str() {
            Ok((first_wallet, second_wallet))
        } else {
            Ok((second_wallet, first_wallet))
        }
    }

    pub async fn execute_transfer(
        &self,
        from_address: &Address,
        to_address: &Address,
        amount: u128,
    ) -> Result<()> {
        // 1. Guard Clause
        if from_address == to_address {
            return Err(anyhow!("Cannot transfer to the same account"));
        }

        // 2. Start the Transaction
        let mut tx = self.provider.begin_transaction().await?;

        // 3. Orchestrate Resource Acquisition (Using our helper!)
        let (mut sender, mut receiver) = self
            .lock_and_fetch_pair(&mut tx, from_address, to_address)
            .await?;

        // 4. Domain Logic (The "Currency Gate")
        let currency = *sender.currency(); // Using Copy!
        let transfer_balance = Balance::new(amount);

        sender
            .withdraw(transfer_balance, currency)
            .map_err(|e| anyhow!(e))?;
        receiver
            .deposit(transfer_balance, currency)
            .map_err(|e| anyhow!(e))?;

        // 5. Persistence
        self.repo.save(&mut tx, &sender).await?;
        self.repo.save(&mut tx, &receiver).await?;

        // 6. Commit
        tx.commit().await?; // Now the compiler knows this exists!

        Ok(())
    }
}
