'use client';

import { useState } from 'react';
import { Attestation } from '@/lib/contract';

interface AttestationListProps {
  attestations: Attestation[];
  isLoading: boolean;
}

export default function AttestationList({ attestations, isLoading }: AttestationListProps) {
  const [filter, setFilter] = useState<'all' | 'active' | 'revoked'>('all');

  const filteredAttestations = attestations.filter((att) => {
    if (filter === 'active') return !att.revoked;
    if (filter === 'revoked') return att.revoked;
    return true;
  });

  const formatTimestamp = (timestamp: bigint) => {
    return new Date(Number(timestamp) * 1000).toLocaleString();
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-6)}`;
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex gap-2 mb-6">
        <button
          onClick={() => setFilter('all')}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            filter === 'all'
              ? 'bg-indigo-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
          }`}
        >
          All ({attestations.length})
        </button>
        <button
          onClick={() => setFilter('active')}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            filter === 'active'
              ? 'bg-indigo-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
          }`}
        >
          Active ({attestations.filter(a => !a.revoked).length})
        </button>
        <button
          onClick={() => setFilter('revoked')}
          className={`px-4 py-2 rounded-lg font-medium transition-colors ${
            filter === 'revoked'
              ? 'bg-indigo-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
          }`}
        >
          Revoked ({attestations.filter(a => a.revoked).length})
        </button>
      </div>

      {filteredAttestations.length === 0 ? (
        <div className="text-center py-12 text-gray-500 dark:text-gray-400">
          No attestations found
        </div>
      ) : (
        <div className="space-y-4">
          {filteredAttestations.map((attestation) => (
            <div
              key={attestation.id.toString()}
              className={`border rounded-lg p-6 ${
                attestation.revoked
                  ? 'border-red-300 bg-red-50 dark:border-red-700 dark:bg-red-900/20'
                  : 'border-gray-300 bg-white dark:border-gray-700 dark:bg-gray-800'
              }`}
            >
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h3 className="text-lg font-semibold mb-1">
                    {attestation.attestation_type}
                  </h3>
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    ID: {attestation.id.toString()}
                  </p>
                </div>
                {attestation.revoked && (
                  <span className="bg-red-600 text-white px-3 py-1 rounded-full text-sm font-medium">
                    Revoked
                  </span>
                )}
              </div>

              <div className="space-y-2 mb-4">
                <div>
                  <span className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Subject:
                  </span>
                  <span className="ml-2 font-mono text-sm">
                    {formatAddress(attestation.subject)}
                  </span>
                </div>
                <div>
                  <span className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Issuer:
                  </span>
                  <span className="ml-2 font-mono text-sm">
                    {formatAddress(attestation.issuer)}
                  </span>
                </div>
                <div>
                  <span className="text-sm font-medium text-gray-600 dark:text-gray-400">
                    Issued:
                  </span>
                  <span className="ml-2 text-sm">
                    {formatTimestamp(attestation.timestamp)}
                  </span>
                </div>
              </div>

              <div className="border-t pt-4 dark:border-gray-700">
                <p className="text-sm text-gray-700 dark:text-gray-300">
                  {attestation.data}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
