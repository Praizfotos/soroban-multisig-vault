'use client';

import { useState } from 'react';
import WalletConnect from '@/components/WalletConnect';
import AttestationList from '@/components/AttestationList';
import IssueAttestationForm from '@/components/IssueAttestationForm';
import { Attestation, AttestationInput } from '@/lib/contract';

export default function Home() {
  const [attestations, setAttestations] = useState<Attestation[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<'view' | 'issue'>('view');

  const handleIssueAttestation = async (input: AttestationInput) => {
    setIsLoading(true);
    try {
      // Contract interaction would go here
      console.log('Issuing attestation:', input);
      // Simulate success
      await new Promise(resolve => setTimeout(resolve, 1000));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                TrustBridge Attestations
              </h1>
              <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                On-chain reputation and verification protocol
              </p>
            </div>
            <WalletConnect />
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="mb-8">
          <div className="border-b border-gray-200 dark:border-gray-700">
            <nav className="-mb-px flex space-x-8">
              <button
                onClick={() => setActiveTab('view')}
                className={`py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'view'
                    ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                }`}
              >
                View Attestations
              </button>
              <button
                onClick={() => setActiveTab('issue')}
                className={`py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'issue'
                    ? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300'
                }`}
              >
                Issue Attestation
              </button>
            </nav>
          </div>
        </div>

        <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
          {activeTab === 'view' ? (
            <AttestationList attestations={attestations} isLoading={isLoading} />
          ) : (
            <div className="max-w-2xl mx-auto">
              <h2 className="text-2xl font-bold mb-6">Issue New Attestation</h2>
              <IssueAttestationForm onSubmit={handleIssueAttestation} isLoading={isLoading} />
            </div>
          )}
        </div>
      </main>

      <footer className="bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 mt-auto">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <p className="text-center text-sm text-gray-500 dark:text-gray-400">
            Built on Stellar Soroban • MIT License
          </p>
        </div>
      </footer>
    </div>
  );
}
