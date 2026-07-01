/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  env: {
    NEXT_PUBLIC_SOROBAN_NETWORK: process.env.NEXT_PUBLIC_SOROBAN_NETWORK || 'testnet',
    NEXT_PUBLIC_TREASURY_CONTRACT_ID: process.env.NEXT_PUBLIC_TREASURY_CONTRACT_ID,
    NEXT_PUBLIC_GOVERNANCE_CONTRACT_ID: process.env.NEXT_PUBLIC_GOVERNANCE_CONTRACT_ID,
    NEXT_PUBLIC_REGISTRY_CONTRACT_ID: process.env.NEXT_PUBLIC_REGISTRY_CONTRACT_ID,
    NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001',
  },
};

export default nextConfig;
