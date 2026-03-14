use soroban_sdk::{symbol_short, Address, Env};

/// Emitted when a new invoice is created.
pub fn invoice_created(
    env: &Env,
    invoice_id: u64,
    freelancer: &Address,
    client: &Address,
    amount: i128,
) {
    env.events().publish(
        (symbol_short!("INVOICE"), symbol_short!("created")),
        (invoice_id, freelancer.clone(), client.clone(), amount),
    );
}

// TODO: Add event emitters for each state transition:
// - fund_invoice    -> emit "INVOICE funded"
// - mark_delivered  -> emit "INVOICE delivered"
// - approve_payment -> emit "INVOICE approved"
// - release_payment -> emit "INVOICE released"
// See: https://github.com/your-org/StarInvoice/issues/7
