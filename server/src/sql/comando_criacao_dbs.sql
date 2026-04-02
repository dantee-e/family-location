CREATE TABLE IF NOT EXISTS  locations (
    id                INTEGER  PRIMARY KEY AUTOINCREMENT,
    verifying_key   BLOB     NOT NULL,                      -- Ed25519 pubkey: 32 bytes
    payload           BLOB     NOT NULL,                      -- ECIES ciphertext (tamanho variável)
    signature         BLOB     NOT NULL,                      -- Ed25519 signature: 64 bytes
    nonce             BLOB     NOT NULL,
    received_at       INTEGER  NOT NULL DEFAULT (unixepoch()) -- Unix timestamp (segundos)
);

CREATE INDEX IF NOT EXISTS idx_locations_user ON locations(verifying_key);

CREATE TABLE IF NOT EXISTS users (
    verifying_key     BLOB     PRIMARY KEY,                   -- Ed25519 pubkey: 32 bytes
    public_key        BLOB     NOT NULL UNIQUE,
    username          TEXT     NOT NULL UNIQUE, 
    registered_at     INTEGER  NOT NULL DEFAULT (unixepoch())
);

CREATE TABLE IF NOT EXISTS access_grants (
    id                    INTEGER  PRIMARY KEY AUTOINCREMENT,
    data_owner_pubkey     BLOB     NOT NULL REFERENCES users(verifying_key),
    grantee_pubkey        BLOB     NOT NULL REFERENCES users(verifying_key),
    reencryption_key      BLOB     NOT NULL,                  -- chave de re-encriptação (tamanho depende da lib)
    grant_signature       BLOB     NOT NULL,                  -- assinatura do dono: 64 bytes
    granted_at            INTEGER  NOT NULL,
    revoked_at            INTEGER,                            -- NULL = ativo, preenchido = revogado

    UNIQUE(data_owner_pubkey, grantee_pubkey)
);
