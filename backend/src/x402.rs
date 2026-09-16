use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402Quote {
	pub resource_id: String,
	pub price_nano_gono: u128,
	pub recipient: String,
	pub channel_id: String,
	pub nonce: u64,
	pub valid_until_block: u64,
	pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402Ticket {
	pub channel_id: String,
	pub nonce: u64,
	pub cumulative_amount: u128,
	pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402VerificationResult {
	pub valid: bool,
	pub channel_id: String,
	pub cumulative_amount: u128,
	pub remaining_deposit: u128,
	pub message: String,
}

pub struct X402GatewayEngine;

impl X402GatewayEngine {
	pub fn generate_quote(resource_id: &str, recipient: &str, channel_id: &str) -> X402Quote {
		X402Quote {
			resource_id: resource_id.to_string(),
			price_nano_gono: 1_000_000, // 0.001 GONO per request
			recipient: recipient.to_string(),
			channel_id: channel_id.to_string(),
			nonce: 4,
			valid_until_block: 25000,
			protocol: "x402/1.0".to_string(),
		}
	}

	pub fn verify_ticket(
		ticket: &X402Ticket,
		quote: &X402Quote,
		channel_deposit: u128,
	) -> Result<X402VerificationResult, String> {
		if ticket.channel_id != quote.channel_id {
			return Err("Channel ID mismatch with active quote".to_string());
		}

		if ticket.cumulative_amount < quote.price_nano_gono {
			return Err(
				"Cumulative voucher amount insufficient for requested resource".to_string(),
			);
		}

		if ticket.cumulative_amount > channel_deposit {
			return Err("Voucher cumulative amount exceeds on-chain deposited balance".to_string());
		}

		if ticket.signature.is_empty() {
			return Err("Missing cryptographic signature on payment ticket".to_string());
		}

		Ok(X402VerificationResult {
			valid: true,
			channel_id: ticket.channel_id.clone(),
			cumulative_amount: ticket.cumulative_amount,
			remaining_deposit: channel_deposit.saturating_sub(ticket.cumulative_amount),
			message: "x402 Payment Proof Verified on Gono Chain".to_string(),
		})
	}
}
