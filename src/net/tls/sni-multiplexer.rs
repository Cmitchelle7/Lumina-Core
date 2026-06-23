pub struct SniMultiplexer;

impl SniMultiplexer {
    pub fn rewrite_sni(tenant_id: &str, original_sni: &str) -> String {
        format!("{}.{}", tenant_id, original_sni)
    }
    pub fn parse_rewritten_sni(rewritten_sni: &str) -> Option<(String, String)> {
        let dot_pos = rewritten_sni.find('.')?;
        let tenant_id = rewritten_sni[..dot_pos].to_string();
        let original_sni = rewritten_sni[dot_pos + 1..].to_string();
        Some((tenant_id, original_sni))
    }
    pub fn is_valid_tenant_prefix(rewritten_sni: &str, expected_tenant_id: &str) -> bool {
        rewritten_sni.starts_with(&format!("{}.", expected_tenant_id))
    }
}
