#[allow(dead_code)]
pub struct AccessGrants {
    id: i64,
    data_owner_pubkey: Vec<u8>,
    grantee_pubkey: Vec<u8>,
    reencryption_key: Vec<u8>,
    grant_signature: Vec<u8>,
    granted_at: i64,
    revoked_at: i64,
}
