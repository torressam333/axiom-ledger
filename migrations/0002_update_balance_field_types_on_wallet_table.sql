-- Updating the balance type to prevent impedence mismatch
-- since rust is screaming that I cant cast BIGDECIMAL into my
-- expected u128 type for xrp drops :)
ALTER TABLE wallets 
ALTER COLUMN balance TYPE BIGINT;
