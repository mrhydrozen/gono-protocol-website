#[path = "../src/chain.rs"]
mod chain;
#[path = "../src/x402.rs"]
mod x402;

use chain::{GonoChainService, MediaReceiptInput};
use x402::{X402GatewayEngine, X402Ticket};

#[actix_web::test]
async fn test_chain_service_receipts_and_dag() {
	let service = GonoChainService::new("http://127.0.0.1:9944");

	// Check pre-populated whitepaper receipt
	let sample_cid = "bafybeic527zcvqjfgw4v2kmz35d2524v5e63gnhv54m75v7t3n6o45dxeu";
	let receipt = service.get_receipt(sample_cid);
	assert!(receipt.is_some());
	let r = receipt.unwrap();
	assert_eq!(r.verified, true);
	assert!(r.storage_providers.contains(&"Arweave".to_string()));

	// Commit a child revision to test DAG linking
	let child_cid = "bafybeichildrev2387410294719284710294710294719284710294819284".to_string();
	let child_receipt = service.commit_receipt(MediaReceiptInput {
		cid: child_cid.clone(),
		author: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
		c2pa_manifest_uri: Some("ipfs://bafybeimanifestchild".to_string()),
		metadata_hash: None,
		parent_cid: Some(sample_cid.to_string()),
		storage_providers: Some(vec!["IPFS".to_string(), "Filecoin".to_string()]),
	});

	assert_eq!(child_receipt.cid, child_cid);
	assert_eq!(child_receipt.parent_cid, Some(sample_cid.to_string()));

	// Verify parent DAG contains child
	let updated_parent = service.get_receipt(sample_cid).unwrap();
	assert!(updated_parent.revisions.contains(&child_cid));

	// Verify listing
	let list = service.list_receipts();
	assert!(list.len() >= 2);

	// Verify channel lookup
	let ch =
		service.get_channel("0x9a8f23719b48c109247184710294719284710294719284719284710294819284");
	assert!(ch.is_some());

	// Verify node health query in offline mode
	let health = service.get_node_health().await;
	assert_eq!(health.connected, false);
	assert_eq!(health.mode, "embedded_state_cache");
}

#[actix_web::test]
async fn test_sanub_credibility_voting_math() {
	let service = GonoChainService::new("http://127.0.0.1:9944");
	let sample_cid = "bafybeic527zcvqjfgw4v2kmz35d2524v5e63gnhv54m75v7t3n6o45dxeu";

	let initial = service.get_credibility(sample_cid).unwrap();
	assert!(initial.credibility_score > 0.9);

	// Cast 10 positive verifier votes
	for _ in 0..10 {
		service.vote_verifier(sample_cid, true);
	}

	let updated = service.get_credibility(sample_cid).unwrap();
	assert_eq!(
		updated.verifier_votes_positive,
		initial.verifier_votes_positive + 10
	);
	assert!(updated.public_belief > initial.public_belief);
}

#[actix_web::test]
async fn test_x402_quote_and_ticket_verification() {
	let quote = X402GatewayEngine::generate_quote(
		"bafybei_article_001",
		"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		"0xchannel_abc_123",
	);

	assert_eq!(quote.price_nano_gono, 1_000_000);
	assert_eq!(quote.protocol, "x402/1.0");

	// Valid ticket with sufficient amount
	let valid_ticket = X402Ticket {
		channel_id: "0xchannel_abc_123".to_string(),
		nonce: 4,
		cumulative_amount: 5_000_000,
		signature: "0x38471029471092847102983471092837410293847102938471029384710293847"
			.to_string(),
	};

	let result = X402GatewayEngine::verify_ticket(&valid_ticket, &quote, 100_000_000);
	assert!(result.is_ok());
	let res = result.unwrap();
	assert!(res.valid);
	assert_eq!(res.remaining_deposit, 95_000_000);

	// Invalid ticket: insufficient amount
	let underpaid_ticket = X402Ticket {
		channel_id: "0xchannel_abc_123".to_string(),
		nonce: 4,
		cumulative_amount: 500, // less than 1_000_000
		signature: "0x123".to_string(),
	};
	let err = X402GatewayEngine::verify_ticket(&underpaid_ticket, &quote, 100_000_000);
	assert!(err.is_err());

	// Invalid ticket: channel deposit exceeded
	let overdrawn_ticket = X402Ticket {
		channel_id: "0xchannel_abc_123".to_string(),
		nonce: 4,
		cumulative_amount: 200_000_000,
		signature: "0x123".to_string(),
	};
	let err2 = X402GatewayEngine::verify_ticket(&overdrawn_ticket, &quote, 100_000_000);
	assert!(err2.is_err());
}
