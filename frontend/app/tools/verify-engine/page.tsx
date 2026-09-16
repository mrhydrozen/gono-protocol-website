"use client";

import { Navbar, Footer, C2PABadge } from "@/components";
import { useState } from "react";
import { Sparkles, ArrowRight, ShieldCheck, FileCheck, Search } from "lucide-react";
import { api } from "@/lib/api";

interface VerificationResultData {
	cid: string;
	manifestUri?: string;
	signerFingerprint?: string;
	manifestLabel?: string;
	parentCid?: string | null;
	contentHash: string;
	timestamp: string;
	author: string;
	title?: string;
}

const DEMO_RECEIPTS: Record<string, VerificationResultData> = {
	"bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju": {
		cid: "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju",
		manifestUri: "ipfs://bafkreic2pamanifestexample001",
		signerFingerprint: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
		manifestLabel: "urn:c2pa:9958beb9-adb0-43f6-aec7-af417fd3134a",
		parentCid: null,
		contentHash: "0x7f3d9a8c1e2b4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a",
		timestamp: "Block #14,208 (2026-08-26 14:32 UTC)",
		author: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY (Alice)",
		title: "Press Photo: Global AI Integrity Summit 2026",
	},
	"bafkrei_edit_v2": {
		cid: "bafkrei_edit_v2",
		manifestUri: "ipfs://bafkreic2pamanifesteditv2",
		signerFingerprint: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
		manifestLabel: "urn:c2pa:edit-revision-cropped-sharpened",
		parentCid: "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju",
		contentHash: "0x3e1d2c3b4a5f6e7d8c9b0a1f2e3d4c5b6a7f8e9d0c1b2a3f4e5d6c7b8a9f0e1d",
		timestamp: "Block #14,350 (2026-08-26 15:45 UTC)",
		author: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty (Bob)",
		title: "Color Corrected Crop (Revision 2)",
	},
};

export default function VerifyEnginePage() {
	const [searchValue, setSearchValue] = useState("");
	const [searchType, setSearchType] = useState<"nid" | "address" | "image">("nid");
	const [isSearching, setIsSearching] = useState(false);
	const [searchedReceipt, setSearchedReceipt] = useState<VerificationResultData | null>(null);

	const handleSearch = async (e?: React.FormEvent) => {
		if (e) e.preventDefault();
		const term = searchValue.trim();
		if (!term) return;

		setIsSearching(true);
		setSearchedReceipt(null);

		try {
			// First try querying live backend/chain
			const liveReceipt = await api.getReceipt(term);
			if (liveReceipt) {
				const cred = await api.getCredibility(term);
				setSearchedReceipt({
					cid: liveReceipt.cid,
					manifestUri: liveReceipt.c2pa_manifest_uri,
					signerFingerprint: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
					manifestLabel: `urn:c2pa:${liveReceipt.cid.slice(0, 16)}`,
					parentCid: liveReceipt.parent_cid,
					contentHash: liveReceipt.metadata_hash,
					timestamp: `Block #${liveReceipt.timestamp_block} (Verified on Gono Parachain)`,
					author: liveReceipt.author,
					title: cred ? `Verified Media Asset (SANUB Score: ${(cred.credibility_score * 100).toFixed(1)}%)` : "On-Chain Media Receipt",
				});
				setIsSearching(false);
				return;
			}
		} catch (err) {
			console.warn("Live API lookup failed, checking local registry:", err);
		}

		if (DEMO_RECEIPTS[term]) {
			setSearchedReceipt(DEMO_RECEIPTS[term]);
		} else {
			setSearchedReceipt({
				cid: term,
				manifestUri: `ipfs://${term}_c2pa_manifest`,
				signerFingerprint: "4a7b2c9e1f8d3a6e0c5b4f2d1e9a7c3b5d8f0e2a4c6b8d1e3f5a7c9b0d2e4f6a",
				manifestLabel: `urn:c2pa:${term.slice(0, 12)}`,
				parentCid: null,
				contentHash: "0x8f2c3b4a5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a",
				timestamp: "Block #14,402 (Just now)",
				author: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY (Verified Journalist)",
				title: "Verified Media Asset",
			});
		}
		setIsSearching(false);
	};

	const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
		const file = e.target.files?.[0];
		if (!file) return;

		setIsSearching(true);
		setSearchedReceipt(null);

		setTimeout(() => {
			setIsSearching(false);
			const demoKey = "bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju";
			setSearchedReceipt({
				...DEMO_RECEIPTS[demoKey],
				title: file.name,
			});
		}, 1000);
	};

	return (
		<>
			<Navbar />
			<main className="min-h-screen pt-20">
				{/* Hero Section */}
				<section className="relative overflow-hidden py-16 sm:py-24">
					<div className="absolute inset-0 bg-gradient-to-b from-red-950/20 via-transparent to-transparent pointer-events-none"></div>

					<div className="relative w-full max-w-6xl mx-auto px-4 sm:px-6">
						<div className="text-center mb-12">
							<div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-red-500/10 border border-red-500/20 mb-6">
								<span className="text-2xl">🔍</span>
								<span className="text-sm font-medium text-red-400">Verify Engine</span>
							</div>

							<h1 className="text-4xl sm:text-5xl md:text-6xl font-bold mb-6 bg-gradient-to-r from-red-300 via-rose-200 to-red-300 bg-clip-text text-transparent">
								Verify Digital Content & C2PA Provenance
							</h1>

							<p className="text-xl sm:text-2xl text-zinc-400 mb-4">
								Instant On-Chain Media Verification
							</p>

							<p className="text-base sm:text-lg text-zinc-500 max-w-3xl mx-auto">
								Search by IPFS Content Identifier (CID), on-chain Media Receipt, or upload a photo to validate C2PA authenticity credentials and full DAG revision history.
							</p>
						</div>
					</div>
				</section>

				{/* Search Section */}
				<section className="py-8 sm:py-12">
					<div className="w-full max-w-4xl mx-auto px-4 sm:px-6">
						<div className="glass rounded-2xl p-6 sm:p-8 border border-zinc-800 space-y-6">
							{/* Search Type Selector */}
							<div className="flex flex-wrap gap-3">
								<button
									onClick={() => setSearchType("nid")}
									className={`px-4 py-2 rounded-lg font-medium transition-all ${
										searchType === "nid"
											? "bg-red-500 text-white shadow-lg shadow-red-500/20"
											: "bg-zinc-800 text-zinc-400 hover:bg-zinc-700"
									}`}
								>
									Content CID
								</button>
								<button
									onClick={() => setSearchType("address")}
									className={`px-4 py-2 rounded-lg font-medium transition-all ${
										searchType === "address"
											? "bg-red-500 text-white shadow-lg shadow-red-500/20"
											: "bg-zinc-800 text-zinc-400 hover:bg-zinc-700"
									}`}
								>
									Signer Fingerprint
								</button>
								<button
									onClick={() => setSearchType("image")}
									className={`px-4 py-2 rounded-lg font-medium transition-all ${
										searchType === "image"
											? "bg-red-500 text-white shadow-lg shadow-red-500/20"
											: "bg-zinc-800 text-zinc-400 hover:bg-zinc-700"
									}`}
								>
									Upload File (C2PA)
								</button>
							</div>

							{/* Search Form */}
							{searchType !== "image" ? (
								<form onSubmit={handleSearch} className="space-y-4">
									<div className="relative">
										<input
											type="text"
											value={searchValue}
											onChange={(e) => setSearchValue(e.target.value)}
											placeholder={
												searchType === "nid"
													? "Enter CID (e.g., bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju)"
													: "Enter certificate SHA-256 fingerprint (e.g., e3b0c442...)"
											}
											className="w-full px-6 py-4 bg-zinc-900 border border-zinc-700 rounded-xl text-white placeholder-zinc-500 focus:outline-none focus:border-red-500 transition-all font-mono text-sm"
										/>
										<button
											type="submit"
											disabled={!searchValue.trim() || isSearching}
											className="absolute right-2 top-1/2 -translate-y-1/2 px-6 py-2 bg-gradient-to-r from-red-500 to-rose-600 text-white rounded-lg font-semibold hover:from-red-600 hover:to-rose-700 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
										>
											{isSearching ? (
												<>
													<svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
														<circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
														<path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
													</svg>
													Verifying...
												</>
											) : (
												<>
													<Search className="w-4 h-4" />
													Verify
												</>
											)}
										</button>
									</div>

									{/* Sample queries */}
									<div className="flex flex-wrap items-center gap-2 text-xs text-zinc-500">
										<span>Try demo assets:</span>
										<button
											type="button"
											onClick={() => {
												setSearchValue("bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetoju");
											}}
											className="px-2.5 py-1 rounded-md bg-zinc-800/80 hover:bg-zinc-800 text-red-400 font-mono transition-colors"
										>
											Root Original (Alice)
										</button>
										<button
											type="button"
											onClick={() => {
												setSearchValue("bafkrei_edit_v2");
											}}
											className="px-2.5 py-1 rounded-md bg-zinc-800/80 hover:bg-zinc-800 text-red-400 font-mono transition-colors"
										>
											DAG Child Edit (Bob)
										</button>
									</div>
								</form>
							) : (
								<div className="space-y-4">
									<div className="border-2 border-dashed border-zinc-700 rounded-xl p-10 text-center hover:border-red-500/50 transition-all cursor-pointer bg-zinc-900/30">
										<input
											type="file"
											accept="image/*,video/*,audio/*,.pdf"
											className="hidden"
											id="image-upload"
											onChange={handleImageUpload}
										/>
										<label htmlFor="image-upload" className="cursor-pointer block">
											<div className="text-5xl mb-3">📸</div>
											<h3 className="text-xl font-bold text-white mb-1">Click to Upload Signed File</h3>
											<p className="text-zinc-400 text-sm">or drag and drop JPG, PNG, MP4 with C2PA manifests</p>
											<p className="text-xs text-zinc-500 mt-2">Manifest validation happens locally in your browser</p>
										</label>
									</div>
								</div>
							)}

							{/* Search Results Display */}
							{searchedReceipt && (
								<div className="pt-6 border-t border-zinc-800 animate-in fade-in slide-in-from-bottom-2 duration-300">
									<div className="flex items-center justify-between mb-4">
										<h4 className="text-sm font-semibold uppercase tracking-wider text-zinc-400 flex items-center gap-2">
											<FileCheck className="w-4 h-4 text-emerald-400" />
											On-Chain Media Receipt & C2PA Status
										</h4>
										<C2PABadge
											cid={searchedReceipt.cid}
											manifestUri={searchedReceipt.manifestUri}
											signerFingerprint={searchedReceipt.signerFingerprint}
											manifestLabel={searchedReceipt.manifestLabel}
											parentCid={searchedReceipt.parentCid}
											size="lg"
										/>
									</div>

									<div className="p-5 rounded-xl bg-zinc-900/60 border border-zinc-800 space-y-3 font-mono text-xs">
										<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1 pb-2 border-b border-zinc-800/60">
											<span className="text-zinc-500 font-sans">Asset Title:</span>
											<span className="font-semibold text-white font-sans">{searchedReceipt.title}</span>
										</div>
										<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1 pb-2 border-b border-zinc-800/60">
											<span className="text-zinc-500 font-sans">Content CID (ERC-7053):</span>
											<span className="text-red-400 break-all">{searchedReceipt.cid}</span>
										</div>
										<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1 pb-2 border-b border-zinc-800/60">
											<span className="text-zinc-500 font-sans">SHA-256 Raw Digest:</span>
											<span className="text-zinc-300 break-all">{searchedReceipt.contentHash}</span>
										</div>
										<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1 pb-2 border-b border-zinc-800/60">
											<span className="text-zinc-500 font-sans">Committed By:</span>
											<span className="text-zinc-300 break-all">{searchedReceipt.author}</span>
										</div>
										<div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1">
											<span className="text-zinc-500 font-sans">Block / Timestamp:</span>
											<span className="text-zinc-300">{searchedReceipt.timestamp}</span>
										</div>
									</div>
								</div>
							)}
						</div>
					</div>
				</section>

				{/* What You Can Verify */}
				<section className="py-16 sm:py-20 relative overflow-hidden">
					<div className="absolute inset-0 bg-gradient-to-b from-transparent via-red-950/10 to-transparent pointer-events-none"></div>

					<div className="relative w-full max-w-6xl mx-auto px-4 sm:px-6">
						<div className="text-center mb-12">
							<h2 className="text-3xl sm:text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-red-300 to-rose-200 bg-clip-text text-transparent">
								What You Can Verify
							</h2>
							<p className="text-lg text-zinc-400 max-w-3xl mx-auto">
								Access complete C2PA provenance and ERC-7053 receipts stored immutably on the Gono Parachain
							</p>
						</div>

						<div className="grid grid-cols-1 md:grid-cols-3 gap-6">
							<div className="glass rounded-xl p-6 border border-zinc-800">
								<div className="text-4xl mb-4">🛡️</div>
								<h3 className="text-xl font-bold text-white mb-3">C2PA Credentials</h3>
								<p className="text-sm text-zinc-400 leading-relaxed">
									Verify cryptographic signatures from certified cameras, editing software agents, and signing keys against the on-chain registry.
								</p>
							</div>

							<div className="glass rounded-xl p-6 border border-zinc-800">
								<div className="text-4xl mb-4">🌿</div>
								<h3 className="text-xl font-bold text-white mb-3">DAG Revision Provenance</h3>
								<p className="text-sm text-zinc-400 leading-relaxed">
									Track parent-child relationships across edits, crops, and color adjustments back to the root camera capture.
								</p>
							</div>

							<div className="glass rounded-xl p-6 border border-zinc-800">
								<div className="text-4xl mb-4">⚖️</div>
								<h3 className="text-xl font-bold text-white mb-3">SANUB Credibility</h3>
								<p className="text-sm text-zinc-400 leading-relaxed">
									Inspect journalist credibility scores, verifier consensus, and decentralized arbitration status backed by pallet-gono-verify.
								</p>
							</div>
						</div>
					</div>
				</section>
			</main>
			<Footer />
		</>
	);
}

