-- Create the Wallets Table
CREATE TABLE IF NOT EXISTS wallets(
  -- Use XRP address as PK b/c it's unique
  address TEXT PRIMARY KEY,
  -- Need a type that can handle something large like Rust's u128 (drops).
  -- 20 digits deep, no decimals
  balance NUMERIC(20, 0) NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
);

-- Create a Ledger Entries table for audit trails
CREATE TABLE IF NOT EXISTS ledger_entries(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  from_address TEXT REFERENCES wallets(address),
  to_address TEXT REFERENCES wallets(address),
  amount NUMERIC(20,0) NOT NULL,
  entry_type TEXT NOT NULL, -- payment, deposit, etc...
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
