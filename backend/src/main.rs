use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

mod chain;
mod x402;

use chain::{GonoChainService, MediaReceiptInput, NodeHealthInfo};
use x402::{X402GatewayEngine, X402Ticket};

// ================================
// Data Models
// ================================

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
	success: bool,
	data: T,
	timestamp: String,
}

#[derive(Serialize)]
struct HealthResponse {
	status: String,
	version: String,
	uptime_seconds: u64,
	node: NodeHealthInfo,
}

#[derive(Serialize)]
struct StatsResponse {
	assets_registered: String,
	active_users: String,
	partners: u32,
	transactions: String,
	countries: u32,
}

#[derive(Serialize)]
struct Feature {
	id: u32,
	title: String,
	description: String,
	icon: String,
}

#[derive(Serialize, Clone)]
pub struct ArchiveItem {
	pub id: u32,
	pub emoji: String,
	pub category: String,
	pub title: String,
	pub excerpt: String,
	pub date: String,
	pub hash: String,
	pub verified: bool,
}

#[derive(Serialize)]
struct Partner {
	id: u32,
	name: String,
	category: String,
}

#[derive(Serialize)]
struct Product {
	id: u32,
	name: String,
	description: String,
	features: Vec<String>,
	link: String,
}

#[derive(Serialize)]
struct TokenInfo {
	symbol: String,
	name: String,
	total_supply: String,
	circulating_supply: String,
	features: Vec<TokenFeature>,
}

#[derive(Serialize)]
struct TokenFeature {
	icon: String,
	title: String,
	description: String,
}

#[derive(Serialize)]
struct ArchitectureLayer {
	layer: u32,
	name: String,
	description: String,
	components: Vec<String>,
}

#[derive(Serialize)]
struct UseCase {
	id: u32,
	title: String,
	description: String,
	category: String,
	pallets_used: Vec<String>,
}

#[derive(Deserialize)]
pub struct VotePayload {
	pub positive: bool,
}

#[derive(Deserialize)]
pub struct QuoteRequest {
	pub resource_id: String,
	pub recipient: Option<String>,
	pub channel_id: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyTicketRequest {
	pub ticket: X402Ticket,
	pub resource_id: String,
}

// ================================
// App State
// ================================

struct AppState {
	start_time: Instant,
	version: String,
	chain: Arc<GonoChainService>,
}

// ================================
// Helper
// ================================

fn wrap_response<T: Serialize>(data: T) -> ApiResponse<T> {
	ApiResponse {
		success: true,
		data,
		timestamp: chrono::Utc::now().to_rfc3339(),
	}
}

// ================================
// API Handlers — Health & Chain
// ================================

#[get("/api/health")]
async fn health_check(data: web::Data<AppState>) -> impl Responder {
	let node_health = data.chain.get_node_health().await;
	let response = HealthResponse {
		status: "healthy".to_string(),
		version: data.version.clone(),
		uptime_seconds: data.start_time.elapsed().as_secs(),
		node: node_health,
	};
	HttpResponse::Ok().json(wrap_response(response))
}

// ================================
// API Handlers — ERC-7053 Media Receipts
// ================================

#[get("/api/receipts")]
async fn list_receipts(data: web::Data<AppState>) -> impl Responder {
	let receipts = data.chain.list_receipts();
	HttpResponse::Ok().json(wrap_response(receipts))
}

#[get("/api/receipts/{cid}")]
async fn get_receipt(cid: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
	match data.chain.get_receipt(&cid) {
		Some(receipt) => HttpResponse::Ok().json(wrap_response(receipt)),
		None => HttpResponse::NotFound().json(serde_json::json!({
			"success": false,
			"error": "Media receipt not found for CID",
			"cid": cid.into_inner()
		})),
	}
}

#[post("/api/receipts")]
async fn commit_receipt(
	input: web::Json<MediaReceiptInput>,
	data: web::Data<AppState>,
) -> impl Responder {
	let receipt = data.chain.commit_receipt(input.into_inner());
	HttpResponse::Created().json(wrap_response(receipt))
}

// ================================
// API Handlers — SANUB Credibility & Verify
// ================================

#[get("/api/verify/{cid}")]
async fn get_credibility(cid: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
	match data.chain.get_credibility(&cid) {
		Some(cred) => HttpResponse::Ok().json(wrap_response(cred)),
		None => HttpResponse::NotFound().json(serde_json::json!({
			"success": false,
			"error": "No verification evaluation registered for CID",
			"cid": cid.into_inner()
		})),
	}
}

#[post("/api/verify/{cid}/vote")]
async fn vote_verifier(
	cid: web::Path<String>,
	payload: web::Json<VotePayload>,
	data: web::Data<AppState>,
) -> impl Responder {
	match data.chain.vote_verifier(&cid, payload.positive) {
		Some(updated) => HttpResponse::Ok().json(wrap_response(updated)),
		None => HttpResponse::NotFound().json(serde_json::json!({
			"success": false,
			"error": "CID not found for voting",
			"cid": cid.into_inner()
		})),
	}
}

// ================================
// API Handlers — x402 Micropayments
// ================================

#[get("/api/x402/channels/{id}")]
async fn get_channel(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
	match data.chain.get_channel(&id) {
		Some(channel) => HttpResponse::Ok().json(wrap_response(channel)),
		None => HttpResponse::NotFound().json(serde_json::json!({
			"success": false,
			"error": "x402 state channel not found",
			"channel_id": id.into_inner()
		})),
	}
}

#[post("/api/x402/quote")]
async fn create_quote(
	payload: web::Json<QuoteRequest>,
	data: web::Data<AppState>,
) -> impl Responder {
	let recipient = payload
		.recipient
		.clone()
		.unwrap_or_else(|| "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string());
	let channel_id = payload.channel_id.clone().unwrap_or_else(|| {
		"0x9a8f23719b48c109247184710294719284710294719284719284710294819284".to_string()
	});

	let quote = X402GatewayEngine::generate_quote(&payload.resource_id, &recipient, &channel_id);
	let _ = data; // avoid unused warning
	HttpResponse::build(actix_web::http::StatusCode::PAYMENT_REQUIRED)
		.insert_header(("WWW-Authenticate", "X402"))
		.json(wrap_response(quote))
}

#[post("/api/x402/verify-ticket")]
async fn verify_ticket(
	payload: web::Json<VerifyTicketRequest>,
	data: web::Data<AppState>,
) -> impl Responder {
	let quote = X402GatewayEngine::generate_quote(
		&payload.resource_id,
		"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
		&payload.ticket.channel_id,
	);

	let channel_deposit = match data.chain.get_channel(&payload.ticket.channel_id) {
		Some(ch) => ch.deposit_nano_gono,
		None => 100_000_000_000, // fallback dev channel deposit
	};

	match X402GatewayEngine::verify_ticket(&payload.ticket, &quote, channel_deposit) {
		Ok(res) => HttpResponse::Ok().json(wrap_response(res)),
		Err(err) => HttpResponse::BadRequest().json(serde_json::json!({
			"success": false,
			"error": err
		})),
	}
}

// ================================
// API Handlers — Protocol Presentation & Ecosystem
// ================================

#[get("/api/stats")]
async fn get_stats(data: web::Data<AppState>) -> impl Responder {
	let receipts = data.chain.list_receipts();
	let stats = StatsResponse {
		assets_registered: format!("{}+", 10_000 + receipts.len()),
		active_users: "12,450".to_string(),
		partners: 48,
		transactions: format!("{}+", 154_200 + (receipts.len() * 3)),
		countries: 94,
	};
	HttpResponse::Ok().json(wrap_response(stats))
}

#[get("/api/features")]
async fn get_features() -> impl Responder {
	let features = vec![
        Feature {
            id: 1,
            title: "Capture".to_string(),
            description: "Privacy-first asset capture with zk-SNARK proofs. Generate cryptographic hashes and embed C2PA metadata while preserving creator anonymity for high-stakes journalism.".to_string(),
            icon: "capture".to_string(),
        },
        Feature {
            id: 2,
            title: "Store".to_string(),
            description: "Permanent decentralized storage with multi-provider redundancy (IPFS, Arweave, Filecoin, StorJ) backed by dynamic Storage Endowment funding.".to_string(),
            icon: "storage".to_string(),
        },
        Feature {
            id: 3,
            title: "Verify".to_string(),
            description: "Community attestations and analyst reviews powered by the SANUB reputation algorithm (Eq 2-8) and AI oracles for mathematical credibility scoring.".to_string(),
            icon: "verify".to_string(),
        },
        Feature {
            id: 4,
            title: "Certify".to_string(),
            description: "ERC-7053 compliant media receipts on the Gono Parachain. Immutable blockchain indexing creates a global provenance DAG with institutional-grade security.".to_string(),
            icon: "certify".to_string(),
        },
        Feature {
            id: 5,
            title: "Check".to_string(),
            description: "Public auditing and verification via Gono Explorer. Anyone can validate the authenticity, revision DAG, and C2PA manifest of registered digital assets.".to_string(),
            icon: "check".to_string(),
        },
    ];
	HttpResponse::Ok().json(wrap_response(features))
}

#[get("/api/archive")]
async fn get_archive(data: web::Data<AppState>) -> impl Responder {
	let mut archive = vec![
        ArchiveItem {
            id: 1,
            emoji: "🌍".to_string(),
            category: "Environment".to_string(),
            title: "Amazon Rainforest Deforestation Audit".to_string(),
            excerpt: "Satellite imagery and ground sensor telemetry verified on Gono Protocol with C2PA hardware attestation.".to_string(),
            date: "Feb 14, 2026".to_string(),
            hash: "0x7a3f898321da3be523e6fc85e4be68b31a396263b65593ec2fe7b92911b3bc22".to_string(),
            verified: true,
        },
        ArchiveItem {
            id: 2,
            emoji: "🗳️".to_string(),
            category: "Governance".to_string(),
            title: "Democratic Referendum Ballot Records".to_string(),
            excerpt: "Verifiable media receipts ensuring election photojournalism cannot be manipulated or deepfaked.".to_string(),
            date: "Jan 28, 2026".to_string(),
            hash: "0x4e9c12b7a9f81203498172938471928471928471928471928471928471928471".to_string(),
            verified: true,
        },
    ];

	// Merge registered on-chain receipts into archive view
	let live_receipts = data.chain.list_receipts();
	for (i, r) in live_receipts.iter().enumerate() {
		if !archive.iter().any(|a| a.hash == r.metadata_hash) {
			archive.push(ArchiveItem {
				id: (archive.len() + i + 1) as u32,
				emoji: "📰".to_string(),
				category: "Investigative Journalism".to_string(),
				title: format!("Media Receipt: {}", &r.cid[..18]),
				excerpt: format!(
					"Provenance DAG root registered by author {}",
					&r.author[..12]
				),
				date: "Recent Block".to_string(),
				hash: r.metadata_hash.clone(),
				verified: r.verified,
			});
		}
	}

	HttpResponse::Ok().json(wrap_response(archive))
}

#[get("/api/partners")]
async fn get_partners() -> impl Responder {
	let partners = vec![
		Partner {
			id: 1,
			name: "Reuters Guild".to_string(),
			category: "News & Media".to_string(),
		},
		Partner {
			id: 2,
			name: "Polkadot Ecosystem".to_string(),
			category: "Relay Security".to_string(),
		},
		Partner {
			id: 3,
			name: "C2PA Alliance".to_string(),
			category: "Standards".to_string(),
		},
		Partner {
			id: 4,
			name: "Arweave Foundation".to_string(),
			category: "Permanent Storage".to_string(),
		},
		Partner {
			id: 5,
			name: "Filecoin / IPFS".to_string(),
			category: "Decentralized Web".to_string(),
		},
	];
	HttpResponse::Ok().json(wrap_response(partners))
}

#[get("/api/products")]
async fn get_products() -> impl Responder {
	let products = vec![
        Product {
            id: 1,
            name: "Gono Capture SDK".to_string(),
            description: "Mobile and web SDK with C2PA hardware attestation and zero-knowledge personhood proofs.".to_string(),
            features: vec!["C2PA Manifest Signing".to_string(), "ZKP Humanity Proofs".to_string(), "Client-side Hashing".to_string()],
            link: "/docs/capture".to_string(),
        },
        Product {
            id: 2,
            name: "Gono Explorer".to_string(),
            description: "Search and inspect content authenticity, SANUB scores, and DAG revisions in real time.".to_string(),
            features: vec!["CID Lookup".to_string(), "DAG Visualizer".to_string(), "Credibility Breakdown".to_string()],
            link: "/tools".to_string(),
        },
        Product {
            id: 3,
            name: "x402 Micropayment Gateway".to_string(),
            description: "HTTP 402 proxy for autonomous AI agents paying per request via state channels.".to_string(),
            features: vec!["Sub-cent Fees".to_string(), "Instant Verification".to_string(), "Off-chain Tickets".to_string()],
            link: "/docs/x402".to_string(),
        },
    ];
	HttpResponse::Ok().json(wrap_response(products))
}

#[get("/api/token")]
async fn get_token_info() -> impl Responder {
	let token = TokenInfo {
        symbol: "GONO".to_string(),
        name: "Gono Protocol Token".to_string(),
        total_supply: "1,000,000,000 GONO".to_string(),
        circulating_supply: "250,000,000 GONO".to_string(),
        features: vec![
            TokenFeature {
                icon: "gas".to_string(),
                title: "Network Gas".to_string(),
                description: "Fuel for transactions, receipt indexing, and smart state transitions on Gono Parachain.".to_string(),
            },
            TokenFeature {
                icon: "storage".to_string(),
                title: "Storage Endowment".to_string(),
                description: "One-time deposit creating sustainable multi-provider storage yields in perpetuity.".to_string(),
            },
            TokenFeature {
                icon: "governance".to_string(),
                title: "Three-Tier Governance".to_string(),
                description: "Token-weighted Conviction Voting and Journalistic Integrity Council meritocracy.".to_string(),
            },
        ],
    };
	HttpResponse::Ok().json(wrap_response(token))
}

#[get("/api/architecture")]
async fn get_architecture() -> impl Responder {
	let architecture = vec![
        ArchitectureLayer {
            layer: 1,
            name: "Layer 1: Shared Security".to_string(),
            description: "Polkadot Relay Chain shared validator pool providing institutional-grade censorship resistance and XCM messaging.".to_string(),
            components: vec!["Relay Validators".to_string(), "XCM Bridge".to_string(), "Agile Coretime".to_string()],
        },
        ArchitectureLayer {
            layer: 2,
            name: "Layer 2: Gono Execution Rail".to_string(),
            description: "Substrate Parachain managing ERC-7053 Media Receipts, DAG versioning, and state transitions.".to_string(),
            components: vec!["pallet-gono-store".to_string(), "pallet-gono-verify".to_string(), "pallet-gono-privacy".to_string(), "pallet-gono-x402".to_string()],
        },
        ArchitectureLayer {
            layer: 3,
            name: "Layer 3: Sovereign Governance".to_string(),
            description: "Conviction Voting (Tier 1), Journalistic Integrity Council (Tier 2), and Kleros Dispute Escalation (Tier 3).".to_string(),
            components: vec!["Democracy".to_string(), "Reputation Council".to_string(), "Kleros Bridge".to_string()],
        },
        ArchitectureLayer {
            layer: 4,
            name: "Layer 4: Application & Commerce".to_string(),
            description: "Decentralized APIs, C2PA Capture SDK, x402 Micropayment Gateway, and Web3 Explorer.".to_string(),
            components: vec!["C2PA Bridge".to_string(), "x402 Proxy".to_string(), "Gono Moncho dApp".to_string()],
        },
    ];
	HttpResponse::Ok().json(wrap_response(architecture))
}

#[get("/api/use-cases")]
async fn get_use_cases() -> impl Responder {
	let use_cases = vec![
        UseCase {
            id: 1,
            title: "Decentralized Journalism".to_string(),
            description: "Whistleblower reporting with zk-SNARK source protection, C2PA metadata, and SANUB credibility consensus.".to_string(),
            category: "Media".to_string(),
            pallets_used: vec!["Capture".to_string(), "Privacy (ZKP)".to_string(), "Store".to_string(), "Verify".to_string(), "Certify".to_string()],
        },
        UseCase {
            id: 2,
            title: "Autonomous AI Commerce".to_string(),
            description: "AI agents purchasing verified high-quality datasets using HTTP 402 micropayment state channels.".to_string(),
            category: "AI Economy".to_string(),
            pallets_used: vec!["x402 Micropayments".to_string(), "Certify".to_string(), "Store".to_string()],
        },
        UseCase {
            id: 3,
            title: "Enterprise Brand & Content Provenance".to_string(),
            description: "Defending intellectual property and authentic photography against generative deepfakes with immutable DAG history.".to_string(),
            category: "Enterprise".to_string(),
            pallets_used: vec!["Capture".to_string(), "Certify".to_string(), "Check".to_string()],
        },
    ];
	HttpResponse::Ok().json(wrap_response(use_cases))
}

// ================================
// Main Server
// ================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

	let rpc_url =
		std::env::var("GONO_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:9944".to_string());
	let chain_service = Arc::new(GonoChainService::new(&rpc_url));

	let app_state = web::Data::new(AppState {
		start_time: Instant::now(),
		version: "0.1.0".to_string(),
		chain: chain_service,
	});

	println!("=========================================================");
	println!("  Gono Protocol Backend API v0.1.0                      ");
	println!("  Live Provenance, Verification & x402 Micropayments    ");
	println!("=========================================================");
	println!("  Listening at: http://127.0.0.1:8080                    ");
	println!(
		"  Node RPC Target: {}                                   ",
		rpc_url
	);
	println!("---------------------------------------------------------");
	println!("  Endpoints:");
	println!("    GET  /api/health                - Node & service health");
	println!("    GET  /api/receipts              - List Media Receipts");
	println!("    GET  /api/receipts/{{cid}}        - Get Media Receipt");
	println!("    POST /api/receipts              - Commit Media Receipt");
	println!("    GET  /api/verify/{{cid}}          - SANUB Credibility");
	println!("    POST /api/verify/{{cid}}/vote     - Verifier Vote");
	println!("    GET  /api/x402/channels/{{id}}    - x402 State Channel");
	println!("    POST /api/x402/quote            - HTTP 402 Quote");
	println!("    POST /api/x402/verify-ticket    - Verify Payment Ticket");
	println!("    GET  /api/archive               - Provenance Archive");
	println!("    GET  /api/stats                 - Network Stats");
	println!("=========================================================");

	HttpServer::new(move || {
		let cors = Cors::default()
			.allow_any_origin()
			.allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
			.allowed_headers(vec![
				actix_web::http::header::AUTHORIZATION,
				actix_web::http::header::ACCEPT,
				actix_web::http::header::CONTENT_TYPE,
			])
			.max_age(3600);

		App::new()
			.app_data(app_state.clone())
			.wrap(cors)
			.wrap(middleware::Logger::default())
			.wrap(middleware::Compress::default())
			// Core Gono Protocol Endpoints
			.service(health_check)
			.service(list_receipts)
			.service(get_receipt)
			.service(commit_receipt)
			.service(get_credibility)
			.service(vote_verifier)
			.service(get_channel)
			.service(create_quote)
			.service(verify_ticket)
			// Ecosystem & Presentation Endpoints
			.service(get_stats)
			.service(get_features)
			.service(get_archive)
			.service(get_partners)
			.service(get_products)
			.service(get_token_info)
			.service(get_architecture)
			.service(get_use_cases)
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
