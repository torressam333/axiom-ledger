-- Add currency column to wallets table so we can store more than just xrp
-- but defaulting to xrp since this will be xrpl driven
ALTER TABLE wallets ADD COLUMN currency TEXT NOT NULL DEFAULT 'XRP';
