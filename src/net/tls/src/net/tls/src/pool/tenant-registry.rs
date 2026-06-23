use std::collections::HashMap;

pub struct TenantEntry {
    pub tenant_id: String,
    pub pool_id: String,
    pub max_sessions: usize,
}

pub struct TenantRegistry {
    tenants: HashMap<String, TenantEntry>,
}

impl TenantRegistry {
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
        }
    }

    pub fn register(&mut self, tenant_id: &str, pool_id: &str, max_sessions: usize) {
        self.tenants.insert(
            tenant_id.to_string(),
            TenantEntry {
                tenant_id: tenant_id.to_string(),
                pool_id: pool_id.to_string(),
                max_sessions,
            },
        );
    }

    pub fn lookup(&self, tenant_id: &str) -> Option<&TenantEntry> {
        self.tenants.get(tenant_id)
    }

    pub fn verify_tenant(&self, tenant_id: &str) -> bool {
        self.tenants.contains_key(tenant_id)
    }
}
