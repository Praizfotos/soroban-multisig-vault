'use client';

import { useState, useEffect } from 'react';
import { FreighterWallet, WalletState } from '@/lib/wallet';

export default function WalletConnect() {
  const [walletState, setWalletState] = useState<WalletState>({
    isConnected: false,
    publicKey: null,
    network: null,
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    checkWalletConnection();
  }, []);

  const checkWalletConnection = async () => {
    const state = await FreighterWallet.getWalletState();
    setWalletState(state);
  };

  const handleConnect = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const publicKey = await FreighterWallet.connect();
      const network = await FreighterWallet.getNetwork();

      setWalletState({
        isConnected: true,
        publicKey,
        network,
      });
    } catch (err) {
      setError('Failed to connect to Freighter wallet. Please install the extension.');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const formatPublicKey = (key: string) => {
    return `${key.slice(0, 6)}...${key.slice(-6)}`;
  };

  return (
    <div className="flex items-center gap-4">
      {walletState.isConnected && walletState.publicKey ? (
        <div className="flex items-center gap-3 bg-gray-100 dark:bg-gray-800 px-4 py-2 rounded-lg">
          <div className="flex flex-col">
            <span className="text-xs text-gray-500 dark:text-gray-400">
              {walletState.network}
            </span>
            <span className="font-mono text-sm font-medium">
              {formatPublicKey(walletState.publicKey)}
            </span>
          </div>
          <div className="w-2 h-2 bg-green-500 rounded-full"></div>
        </div>
      ) : (
        <button
          onClick={handleConnect}
          disabled={isLoading}
          className="bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-2 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isLoading ? 'Connecting...' : 'Connect Wallet'}
        </button>
      )}
      
      {error && (
        <div className="text-red-500 text-sm">
          {error}
        </div>
      )}
    </div>
  );
}
