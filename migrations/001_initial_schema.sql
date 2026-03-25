-- Create the Wallets table
CREATE TABLE IF NOT EXISTS wallets (
    address TEXT PRIMARY KEY,
    balance NUMERIC(20, 0) NOT NULL DEFAULT 0,
    -- Status replaces "Deletion"
    is_active BOOLEAN NOT NULL DEFAULT TRUE, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CREATE THE "IMMUTABILITY GUARD"
-- This trigger prevents ANYONE (even a bug in the code) from deleting a row.
CREATE OR REPLACE FUNCTION block_wallet_deletion()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Manual deletion of wallet records is prohibited for ledger integrity.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_no_delete_wallet
BEFORE DELETE ON wallets
FOR EACH ROW EXECUTE FUNCTION block_wallet_deletion();

-- Ledger Entries: Append-Only by Design
CREATE TABLE IF NOT EXISTS ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_address TEXT REFERENCES wallets(address),
    to_address TEXT REFERENCES wallets(address),
    amount NUMERIC(20, 0) NOT NULL,
    entry_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
