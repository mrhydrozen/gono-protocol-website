use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealthInfo {
	pub connected: bool,
	pub mode: String,
	pub peers: u32,
	pub is_syncing: bool,
	pub version: String,
	pub best_block: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaReceiptData {
	pub cid: String,
	pub author: String,
	pub c2pa_manifest_uri: Option<String>,
	pub metadata_hash: String,
	pub timestamp_block: u64,
	pub parent_cid: Option<String>,
	pub revisions: Vec<String>,
	pub verified: bool,
	pub storage_providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaReceiptInput {
	pub cid: String,
	pub author: String,
	pub c2pa_manifest_uri: Option<String>,
	pub metadata_hash: Option<String>,
	pub parent_cid: Option<String>,
	pub storage_providers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanubCredibilityData {
	pub cid: String,
	pub credibility_score: f64, // C_n in [0.0, 1.0]
	pub public_belief: f64,     // B_n
	pub news_importance: f64,   // I_n
	pub belief_sigmoid: f64,    // S(B_n)
	pub reporter_credit: f64,   // C_r
	pub verifier_votes_positive: u32,
	pub verifier_votes_negative: u32,
	pub analyst_verdict_positive: u32,
	pub analyst_verdict_negative: u32,
	pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402ChannelData {
	pub channel_id: String,
	pub sender: String,
	pub recipient: String,
	pub deposit_nano_gono: u128,
	pub settled_nano_gono: u128,
	pub nonce: u64,
	pub expiration_block: u64,
	pub state: String, // "Open", "Challenged", "Settled"
}

pub struct GonoChainService {
	rpc_url: String,
	http_client: reqwest::Client,
	receipts: RwLock<HashMap<String, MediaReceiptData>>,
	credibility: RwLock<HashMap<String, SanubCredibilityData>>,
	channels: RwLock<HashMap<String, X402ChannelData>>,
}

impl GonoChainService {
	pub fn new(rpc_url: &str) -> Self {
		let mut receipts = HashMap::new();
		let mut credibility = HashMap::new();
		let mut channels = HashMap::new();

		// Populate initial representative provenance data matching Gono whitepaper examples
		let cid_war_report =
			"bafybeic527zcvqjfgw4v2kmz35d2524v5e63gnhv54m75v7t3n6o45dxeu".to_string();
		receipts.insert(
			cid_war_report.clone(),
			MediaReceiptData {
				cid: cid_war_report.clone(),
				author: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(), // Alice
				c2pa_manifest_uri: Some("ipfs://bafybeih523manifesturiwarreport001".to_string()),
				metadata_hash: "0x7a3f898321da3be523e6fc85e4be68b31a396263b65593ec2fe7b92911b3bc22"
					.to_string(),
				timestamp_block: 1040,
				parent_cid: None,
				revisions: vec![
					"bafybeid623zcvqjfgw4v2kmz35d2524v5e63gnhv54m75v7t3n6o45dxev".to_string(),
				],
				verified: true,
				storage_providers: vec!["IPFS".to_string(), "Arweave".to_string()],
			},
		);

		credibility.insert(
			cid_war_report.clone(),
			SanubCredibilityData {
				cid: cid_war_report.clone(),
				credibility_score: 0.942,
				public_belief: 0.88,
				news_importance: 0.95,
				belief_sigmoid: 0.91,
				reporter_credit: 0.92,
				verifier_votes_positive: 42,
				verifier_votes_negative: 2,
				analyst_verdict_positive: 7,
				analyst_verdict_negative: 0,
				status: "Finalized".to_string(),
			},
		);

		let sample_channel_id =
			"0x9a8f23719b48c109247184710294719284710294719284719284710294819284".to_string();
		channels.insert(
			sample_channel_id.clone(),
			X402ChannelData {
				channel_id: sample_channel_id,
				sender: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(), // Bob (AI Agent)
				recipient: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(), // Alice (Publisher)
				deposit_nano_gono: 100_000_000_000,                                     // 100 GONO
				settled_nano_gono: 5_000_000,                                           // 0.005 GONO
				nonce: 3,
				expiration_block: 24400,
				state: "Open".to_string(),
			},
		);

		Self {
			rpc_url: rpc_url.to_string(),
			http_client: reqwest::Client::builder()
				.timeout(Duration::from_millis(800))
				.build()
				.unwrap_or_default(),
			receipts: RwLock::new(receipts),
			credibility: RwLock::new(credibility),
			channels: RwLock::new(channels),
		}
	}

	pub async fn get_node_health(&self) -> NodeHealthInfo {
		let payload = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "system_health",
			"params": [],
			"id": 1
		});

		match self
			.http_client
			.post(&self.rpc_url)
			.json(&payload)
			.send()
			.await
		{
			Ok(resp) if resp.status().is_success() => {
				if let Ok(json) = resp.json::<serde_json::Value>().await {
					let peers = json["result"]["peers"].as_u64().unwrap_or(0) as u32;
					let is_syncing = json["result"]["isSyncing"].as_bool().unwrap_or(false);

					// Fetch block header
					let head_payload = serde_json::json!({
						"jsonrpc": "2.0",
						"method": "chain_getHeader",
						"params": [],
						"id": 2
					});
					let best_block = match self
						.http_client
						.post(&self.rpc_url)
						.json(&head_payload)
						.send()
						.await
					{
						Ok(h_resp) => match h_resp.json::<serde_json::Value>().await {
							Ok(h_json) => {
								let num_hex = h_json["result"]["number"].as_str().unwrap_or("0x0");
								u64::from_str_radix(num_hex.trim_start_matches("0x"), 16)
									.unwrap_or(1)
							}
							Err(_) => 1,
						},
						Err(_) => 1,
					};

					return NodeHealthInfo {
						connected: true,
						mode: "live_parachain_rpc".to_string(),
						peers,
						is_syncing,
						version: "gono-runtime-2606.0.0".to_string(),
						best_block,
					};
				}
			}
			_ => {}
		}

		// Fallback when RPC node is offline or during testing
		NodeHealthInfo {
			connected: false,
			mode: "embedded_state_cache".to_string(),
			peers: 0,
			is_syncing: false,
			version: "gono-runtime-dev".to_string(),
			best_block: 1042,
		}
	}

	pub fn get_receipt(&self, cid: &str) -> Option<MediaReceiptData> {
		let map = self.receipts.read().ok()?;
		map.get(cid).cloned()
	}

	pub fn list_receipts(&self) -> Vec<MediaReceiptData> {
		let map = self.receipts.read().unwrap_or_else(|e| e.into_inner());
		map.values().cloned().collect()
	}

	pub fn commit_receipt(&self, input: MediaReceiptInput) -> MediaReceiptData {
		let mut map = self.receipts.write().unwrap_or_else(|e| e.into_inner());

		let metadata_hash = input.metadata_hash.unwrap_or_else(|| {
			use sha2::{Digest, Sha256};
			let mut hasher = Sha256::new();
			hasher.update(input.cid.as_bytes());
			format!("0x{}", hex::encode(hasher.finalize()))
		});

		let receipt = MediaReceiptData {
			cid: input.cid.clone(),
			author: input.author,
			c2pa_manifest_uri: input.c2pa_manifest_uri,
			metadata_hash,
			timestamp_block: 1043,
			parent_cid: input.parent_cid.clone(),
			revisions: Vec::new(),
			verified: true,
			storage_providers: input
				.storage_providers
				.unwrap_or_else(|| vec!["IPFS".to_string()]),
		};

		// If parent exists, register this CID in parent revisions DAG
		if let Some(parent) = &input.parent_cid {
			if let Some(p_entry) = map.get_mut(parent) {
				if !p_entry.revisions.contains(&input.cid) {
					p_entry.revisions.push(input.cid.clone());
				}
			}
		}

		map.insert(input.cid.clone(), receipt.clone());

		// Also register initial baseline credibility in SANUB table
		if let Ok(mut cred_map) = self.credibility.write() {
			cred_map
				.entry(input.cid.clone())
				.or_insert(SanubCredibilityData {
					cid: input.cid,
					credibility_score: 0.85,
					public_belief: 0.75,
					news_importance: 0.80,
					belief_sigmoid: 0.82,
					reporter_credit: 0.88,
					verifier_votes_positive: 1,
					verifier_votes_negative: 0,
					analyst_verdict_positive: 1,
					analyst_verdict_negative: 0,
					status: "ActiveEvaluation".to_string(),
				});
		}

		receipt
	}

	pub fn get_credibility(&self, cid: &str) -> Option<SanubCredibilityData> {
		let map = self.credibility.read().ok()?;
		map.get(cid).cloned()
	}

	pub fn vote_verifier(&self, cid: &str, positive: bool) -> Option<SanubCredibilityData> {
		let mut map = self.credibility.write().ok()?;
		let entry = map.get_mut(cid)?;
		if positive {
			entry.verifier_votes_positive += 1;
		} else {
			entry.verifier_votes_negative += 1;
		}

		// Recalculate SANUB Public Belief: B_n = N_true / (N_true + N_false)
		let total = entry.verifier_votes_positive + entry.verifier_votes_negative;
		if total > 0 {
			entry.public_belief = (entry.verifier_votes_positive as f64) / (total as f64);
			// Sigmoid: S(B_n) = 1 / (1 + e^(-k*(B_n - 0.5))) with k=6
			let k = 6.0;
			let exponent = -k * (entry.public_belief - 0.5);
			entry.belief_sigmoid = 1.0 / (1.0 + exponent.exp());
			// Content Credibility: C_n = 0.5 * S(B_n) + 0.5 * C_r
			entry.credibility_score = 0.5 * entry.belief_sigmoid + 0.5 * entry.reporter_credit;
		}
		Some(entry.clone())
	}

	pub fn get_channel(&self, channel_id: &str) -> Option<X402ChannelData> {
		let map = self.channels.read().ok()?;
		map.get(channel_id).cloned()
	}
}
