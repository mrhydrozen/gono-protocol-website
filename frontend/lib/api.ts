// API configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

// Fallback mode for when backend is not available (e.g., during Vercel build)
const USE_FALLBACK = process.env.NEXT_PUBLIC_USE_FALLBACK === 'true';

export interface ApiResponse<T> {
    success: boolean;
    data: T;
    timestamp: string;
}

export interface Stats {
    assets_registered: string;
    active_users: string;
    partners: number;
    transactions: string;
    countries: number;
}

export interface Feature {
    id: number;
    title: string;
    description: string;
    icon: string;
}

export interface ArchiveItem {
    id: number;
    emoji: string;
    category: string;
    title: string;
    excerpt: string;
    date: string;
    hash: string;
    verified: boolean;
}

export interface Partner {
    id: number;
    name: string;
    category: string;
}

export interface Product {
    id: number;
    name: string;
    description: string;
    features: string[];
    link: string;
}

export interface TokenInfo {
    symbol: string;
    name: string;
    total_supply: string;
    circulating_supply: string;
    features: TokenFeature[];
}

export interface TokenFeature {
    icon: string;
    title: string;
    description: string;
}

export interface ArchitectureLayer {
    layer: number;
    name: string;
    description: string;
    components: string[];
}

export interface UseCase {
    id: number;
    title: string;
    description: string;
    category: string;
    pallets_used: string[];
}

export interface MediaReceiptData {
    cid: string;
    author: string;
    c2pa_manifest_uri?: string;
    metadata_hash: string;
    timestamp_block: number;
    parent_cid?: string | null;
    revisions: string[];
    verified: boolean;
    storage_providers: string[];
}

export interface SanubCredibilityData {
    cid: string;
    credibility_score: number;
    public_belief: number;
    news_importance: number;
    belief_sigmoid: number;
    reporter_credit: number;
    verifier_votes_positive: number;
    verifier_votes_negative: number;
    analyst_verdict_positive: number;
    analyst_verdict_negative: number;
    status: string;
}

export interface X402Quote {
    resource_id: string;
    price_nano_gono: number;
    recipient: string;
    channel_id: string;
    nonce: number;
    valid_until_block: number;
    protocol: string;
}

async function fetchApi<T>(endpoint: string): Promise<T | null> {
    // Skip API calls during build if backend is not available
    if (USE_FALLBACK || typeof window === 'undefined') {
        console.warn('API call skipped (using fallback data):', endpoint);
        return null;
    }
    
    try {
        const response = await fetch(`${API_BASE_URL}${endpoint}`, {
            cache: 'no-store',
        });
        if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
        }
        const json: ApiResponse<T> = await response.json();
        return json.data;
    } catch (error) {
        console.error('Failed to fetch:', endpoint, error);
        return null;
    }
}

export const api = {
    getStats: () => fetchApi<Stats>('/api/stats'),
    getFeatures: () => fetchApi<Feature[]>('/api/features'),
    getArchive: () => fetchApi<ArchiveItem[]>('/api/archive'),
    getPartners: () => fetchApi<Partner[]>('/api/partners'),
    getProducts: () => fetchApi<Product[]>('/api/products'),
    getToken: () => fetchApi<TokenInfo>('/api/token'),
    getArchitecture: () => fetchApi<ArchitectureLayer[]>('/api/architecture'),
    getUseCases: () => fetchApi<UseCase[]>('/api/use-cases'),
    // Gono On-chain & Verification Endpoints
    getReceipt: (cid: string) => fetchApi<MediaReceiptData>(`/api/receipts/${encodeURIComponent(cid)}`),
    listReceipts: () => fetchApi<MediaReceiptData[]>('/api/receipts'),
    getCredibility: (cid: string) => fetchApi<SanubCredibilityData>(`/api/verify/${encodeURIComponent(cid)}`),
    getX402Quote: (resourceId: string) => fetchApi<X402Quote>('/api/x402/quote'),
};
