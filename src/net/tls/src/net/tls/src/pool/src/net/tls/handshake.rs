use super::session_cache::SessionCache;
use super::sni_multiplexer::SniMultiplexer;

pub struct TlsHandshake {
    cache: SessionCache,
}

impl TlsHandshake {
    pub fn new() -> Self {
        Self {
            cache: SessionCache::new(),
        }
    }

    pub fn handle_client_hello(
        &mut self,
        tenant_id: &str,
        original_sni: &str,
    ) -> Result<Vec<u8>, HandshakeError> {
        let rewritten_sni = SniMultiplexer::rewrite_sni(tenant_id, original_sni);

        if let Some(session) = self.cache.lookup(tenant_id, original_sni) {
            if session.tenant_id == tenant_id {
                return Ok(session.session_data.clone());
            }
            return Err(HandshakeError::TenantMismatch);
        }

        let session_data = vec![0u8; 32];
        self.cache.store(tenant_id, original_sni, session_data.clone(), None)
            .map_err(|_| HandshakeError::CacheFull)?;

        Ok(session_data)
    }

    pub fn verify_session_resumption(
        &mut self,
        rewritten_sni: &str,
        authenticated_tenant: &str,
    ) -> Result<bool, HandshakeError> {
        if let Some((tenant_id, original_sni)) = SniMultiplexer::parse_rewritten_sni(rewritten_sni) {
            if tenant_id != authenticated_tenant {
                return Err(HandshakeError::TenantMismatch);
            }
            return Ok(self.cache.lookup(&tenant_id, &original_sni).is_some());
        }
        Err(HandshakeError::InvalidSni)
    }

    pub fn collision_attempts(&self) -> u64 {
        self.cache.collision_attempts
    }
}

#[derive(Debug)]
pub enum HandshakeError {
    TenantMismatch,
    CacheFull,
    InvalidSni,
    SessionExpired,
}
