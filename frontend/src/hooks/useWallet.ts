import { create } from 'zustand';
import { isConnected, getPublicKey, signTransaction, requestAccess } from '@stellar/freighter-api';

interface WalletState {
  connected: boolean;
  publicKey: string | null;
  loading: boolean;
  error: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  signTransaction: (xdr: string) => Promise<string>;
}

export const useWallet = create<WalletState>((set, get) => ({
  connected: false,
  publicKey: null,
  loading: false,
  error: null,

  connect: async () => {
    set({ loading: true, error: null });
    
    try {
      // Check if Freighter is installed
      const connected = await isConnected();
      
      if (!connected) {
        throw new Error('Freighter wallet not found. Please install Freighter extension.');
      }

      // Request access
      await requestAccess();

      // Get public key
      const publicKey = await getPublicKey();

      set({ 
        connected: true, 
        publicKey,
        loading: false 
      });
    } catch (error: any) {
      set({ 
        error: error.message || 'Failed to connect wallet',
        loading: false,
        connected: false,
        publicKey: null
      });
    }
  },

  disconnect: () => {
    set({ 
      connected: false, 
      publicKey: null,
      error: null 
    });
  },

  signTransaction: async (xdr: string) => {
    const { connected } = get();
    
    if (!connected) {
      throw new Error('Wallet not connected');
    }

    try {
      const signedXdr = await signTransaction(xdr, {
        networkPassphrase: process.env.NEXT_PUBLIC_SOROBAN_NETWORK === 'mainnet' 
          ? 'Public Global Stellar Network ; September 2015'
          : 'Test SDF Network ; September 2015'
      });

      return signedXdr;
    } catch (error: any) {
      throw new Error(error.message || 'Failed to sign transaction');
    }
  },
}));
