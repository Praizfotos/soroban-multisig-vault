'use client';

import { useState } from 'react';
import { AttestationInput } from '@/lib/contract';

interface IssueAttestationFormProps {
  onSubmit: (input: AttestationInput) => Promise<void>;
  isLoading: boolean;
}

export default function IssueAttestationForm({ onSubmit, isLoading }: IssueAttestationFormProps) {
  const [formData, setFormData] = useState<AttestationInput>({
    subject: '',
    attestation_type: '',
    data: '',
  });
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    if (!formData.subject || !formData.attestation_type || !formData.data) {
      setError('All fields are required');
      return;
    }

    if (!formData.subject.startsWith('G') || formData.subject.length !== 56) {
      setError('Invalid Stellar address');
      return;
    }

    try {
      await onSubmit(formData);
      setFormData({
        subject: '',
        attestation_type: '',
        data: '',
      });
    } catch (err) {
      setError('Failed to issue attestation');
      console.error(err);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div>
        <label htmlFor="subject" className="block text-sm font-medium mb-2">
          Subject Address
        </label>
        <input
          type="text"
          id="subject"
          value={formData.subject}
          onChange={(e) => setFormData({ ...formData, subject: e.target.value })}
          placeholder="GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          disabled={isLoading}
        />
      </div>

      <div>
        <label htmlFor="attestation_type" className="block text-sm font-medium mb-2">
          Attestation Type
        </label>
        <input
          type="text"
          id="attestation_type"
          value={formData.attestation_type}
          onChange={(e) => setFormData({ ...formData, attestation_type: e.target.value })}
          placeholder="e.g., kyc_verified, contributor, developer"
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          disabled={isLoading}
        />
      </div>

      <div>
        <label htmlFor="data" className="block text-sm font-medium mb-2">
          Attestation Data
        </label>
        <textarea
          id="data"
          value={formData.data}
          onChange={(e) => setFormData({ ...formData, data: e.target.value })}
          placeholder="Additional information about this attestation"
          rows={4}
          className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          disabled={isLoading}
        />
      </div>

      {error && (
        <div className="bg-red-100 dark:bg-red-900/30 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-400 px-4 py-3 rounded">
          {error}
        </div>
      )}

      <button
        type="submit"
        disabled={isLoading}
        className="w-full bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-3 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? 'Issuing...' : 'Issue Attestation'}
      </button>
    </form>
  );
}
