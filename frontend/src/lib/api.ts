import axios from 'axios';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001';

export const apiClient = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export interface Treasury {
  id: string;
  name: string;
  creator: string;
  signers: string[];
  threshold: number;
  created_at: string;
  paused: boolean;
}

export interface Proposal {
  id: string;
  treasury_id: string;
  proposal_type: string;
  proposer: string;
  status: 'pending' | 'approved' | 'rejected' | 'executed';
  approvals: number;
  rejections: number;
  created_at: string;
  expires_at: string;
  executed: boolean;
}

export interface Vote {
  proposal_id: string;
  voter: string;
  vote_type: 'approve' | 'reject';
  timestamp: string;
}

export const api = {
  // Treasury endpoints
  getTreasuries: async (): Promise<Treasury[]> => {
    const { data } = await apiClient.get('/treasuries');
    return data;
  },

  getTreasury: async (id: string): Promise<Treasury> => {
    const { data } = await apiClient.get(`/treasuries/${id}`);
    return data;
  },

  getTreasuriesByCreator: async (creator: string): Promise<Treasury[]> => {
    const { data } = await apiClient.get(`/treasuries/creator/${creator}`);
    return data;
  },

  // Proposal endpoints
  getProposals: async (treasuryId?: string): Promise<Proposal[]> => {
    const url = treasuryId ? `/proposals?treasury_id=${treasuryId}` : '/proposals';
    const { data } = await apiClient.get(url);
    return data;
  },

  getProposal: async (id: string): Promise<Proposal> => {
    const { data } = await apiClient.get(`/proposals/${id}`);
    return data;
  },

  // Vote endpoints
  getVotes: async (proposalId: string): Promise<Vote[]> => {
    const { data } = await apiClient.get(`/proposals/${proposalId}/votes`);
    return data;
  },

  // Events endpoints
  getEvents: async (treasuryId?: string) => {
    const url = treasuryId ? `/events?treasury_id=${treasuryId}` : '/events';
    const { data } = await apiClient.get(url);
    return data;
  },

  // Stats endpoints
  getStats: async () => {
    const { data } = await apiClient.get('/stats');
    return data;
  },
};
