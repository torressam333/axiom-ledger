use crate::domain::Address;
use crate::domain::Balance;
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
    R: WalletRepository,
    P: TransactionProvider,
{
    pub fn new(repo: R, provider: P) -> Self {
        Self { repo, provider }
    }

    pub async fn execute_transfer(
        &self,
        from_address: &Address,
        to_address: &Address,
        amount: u128,
    ) -> Result<()> {
        // 0. CHECK FOR SELF-TRANSFER
        if from_address.as_str() == to_address.as_str() {
            return Err(anyhow!("Cannot transfer to the same account"));
        }

        // 1. BEGIN THE TX
        let mut tx = self.provider.begin_transaction().await?;

        // 2. DETERMINISTIC ORDERING
        // Guarantees that every thread always requests the locks in EXACT same order
        let (first_addr, second_addr) = if from_address.as_str() < to_address.as_str() {
            (from_address, to_address)
        } else {
            (to_address, from_address)
        };

        // 3. LOCK BOTH ROWS (Pessimistic Locking)
        // Note: We use the &mut tx so both locks are part of the SAME transaction
        let first_wallet = self
            .repo
            .find_by_address_for_update(&mut tx, first_addr)
            .await?
            .ok_or_else(|| anyhow!("Wallet not found: {}", first_addr))?;

        let second_wallet = self
            .repo
            .find_by_address_for_update(&mut tx, second_addr)
            .await?
            .ok_or_else(|| anyhow!("Wallet not found: {}", second_addr))?;

        // 4. NEXT SUB-STEP: Who is the sender?
        // (Since we sorted them, first_wallet might be the receiver!)
        // Map our ordered "first/second" wallets back to "sender/receiver" roles
        let (mut sender, mut receiver) = if first_wallet.address() == from_address.as_str() {
            (first_wallet, second_wallet)
        } else {
            (second_wallet, first_wallet)
        };

        // 5. BUSINESS LOGIC
        // Use domain methods to ensure the math follows business rules.
        if sender.balance() < amount {
            return Err(anyhow!(
                "Insufficient funds: {} has less than {}",
                sender.address(),
                amount
            ));
        }

        // Check currency matches
        let transfer_currency = sender.currency().clone();

        if sender.currency() != receiver.currency() {
            return Err(anyhow!(
                "Multi-asset transfer error: Sender is {:?}, but Receiver is {:?}",
                sender.currency(),
                receiver.currency()
            ));
        }

        // We must update the in-memory state of the objects before saving.
        let transfer_balance = Balance::new(amount);

        sender
            .withdraw(transfer_balance, transfer_currency)
            .map_err(|e| anyhow!(e))?;
        receiver
            .deposit(transfer_balance, transfer_currency)
            .map_err(|e| anyhow!(e))?;

        // 6. PERSISTENCE (Inside the bubble)
        // MUST use the same &mut tx so these updates happen before the lock is released.
        self.repo.save(&mut tx, &sender).await?;
        self.repo.save(&mut tx, &receiver).await?;

        // 7. COMMIT the TX
        // If we get here, everything worked. Commit the transaction.
        // If the code crashed before this line, Postgres would auto-rollback.
        tx.commit().await?;

        Ok(())
    }
}
