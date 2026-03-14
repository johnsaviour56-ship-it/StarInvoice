use soroban_sdk::{contracttype, Address, Env, String};

/// Represents the lifecycle state of an invoice.
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum InvoiceStatus {
    Pending,
    Funded,
    Delivered,
    Approved,
    Completed,
    // TODO: Consider adding Disputed and Cancelled states
    // See: https://github.com/your-org/StarInvoice/issues/5
}

/// Core invoice data structure stored on-chain.
#[contracttype]
#[derive(Clone)]
pub struct Invoice {
    pub id: u64,
    pub freelancer: Address,
    pub client: Address,
    pub amount: i128,
    pub description: String,
    pub status: InvoiceStatus,
    // TODO: Add deadline / expiry field
    // TODO: Add token address field for multi-token support
    // See: https://github.com/your-org/StarInvoice/issues/6
}

#[contracttype]
enum DataKey {
    Invoice(u64),
    InvoiceCount,
}

/// Returns the next available invoice ID and increments the counter.
pub fn next_invoice_id(env: &Env) -> u64 {
    let count: u64 = env
        .storage()
        .instance()
        .get(&DataKey::InvoiceCount)
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&DataKey::InvoiceCount, &(count + 1));
    count
}

pub fn save_invoice(env: &Env, invoice: &Invoice) {
    env.storage()
        .persistent()
        .set(&DataKey::Invoice(invoice.id), invoice);
}

pub fn get_invoice(env: &Env, invoice_id: u64) -> Invoice {
    env.storage()
        .persistent()
        .get(&DataKey::Invoice(invoice_id))
        .expect("Invoice not found")
}
