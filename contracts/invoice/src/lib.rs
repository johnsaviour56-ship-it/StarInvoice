#![no_std]

mod events;
mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub use storage::Invoice;

#[contract]
pub struct InvoiceContract;

#[contractimpl]
impl InvoiceContract {
    /// Creates a new invoice and stores it on-chain.
    /// Returns the generated invoice ID.
    pub fn create_invoice(
        env: Env,
        freelancer: Address,
        client: Address,
        amount: i128,
        description: String,
    ) -> u64 {
        freelancer.require_auth();

        let invoice_id = storage::next_invoice_id(&env);

        let invoice = Invoice {
            id: invoice_id,
            freelancer: freelancer.clone(),
            client: client.clone(),
            amount,
            description,
            status: storage::InvoiceStatus::Pending,
        };

        storage::save_invoice(&env, &invoice);
        events::invoice_created(&env, invoice_id, &freelancer, &client, amount);

        invoice_id
    }

    /// Allows the client to fund the invoice escrow.
    /// TODO: Implement escrow funding logic
    /// - Verify caller is the invoice client
    /// - Transfer `amount` tokens from client to contract
    /// - Update invoice status to Funded
    /// - Emit fund_invoice event
    /// See: https://github.com/your-org/StarInvoice/issues/1
    pub fn fund_invoice(_env: Env, _invoice_id: u64) {
        todo!("fund_invoice not yet implemented")
    }

    /// Allows the freelancer to mark work as delivered.
    /// TODO: Implement delivery marking logic
    /// - Verify caller is the invoice freelancer
    /// - Verify invoice status is Funded
    /// - Update invoice status to Delivered
    /// - Emit mark_delivered event
    /// See: https://github.com/your-org/StarInvoice/issues/2
    pub fn mark_delivered(_env: Env, _invoice_id: u64) {
        todo!("mark_delivered not yet implemented")
    }

    /// Allows the client to approve the delivered work.
    /// TODO: Implement approval logic
    /// - Verify caller is the invoice client
    /// - Verify invoice status is Delivered
    /// - Update invoice status to Approved
    /// - Emit approve_payment event
    /// See: https://github.com/your-org/StarInvoice/issues/3
    pub fn approve_payment(_env: Env, _invoice_id: u64) {
        todo!("approve_payment not yet implemented")
    }

    /// Releases escrowed funds to the freelancer after approval.
    /// TODO: Implement payment release logic
    /// - Verify invoice status is Approved
    /// - Transfer escrowed tokens to freelancer
    /// - Update invoice status to Completed
    /// - Emit release_payment event
    /// See: https://github.com/your-org/StarInvoice/issues/4
    pub fn release_payment(_env: Env, _invoice_id: u64) {
        todo!("release_payment not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_create_invoice() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, InvoiceContract);
        let client = InvoiceContractClient::new(&env, &contract_id);

        let freelancer = Address::generate(&env);
        let payer = Address::generate(&env);
        let description = String::from_str(&env, "Website redesign - Phase 1");

        let invoice_id = client.create_invoice(&freelancer, &payer, &1000, &description);

        assert_eq!(invoice_id, 0);

        // Verify the invoice was stored correctly
        let invoice = storage::get_invoice(&env, invoice_id);
        assert_eq!(invoice.freelancer, freelancer);
        assert_eq!(invoice.client, payer);
        assert_eq!(invoice.amount, 1000);
    }
}
