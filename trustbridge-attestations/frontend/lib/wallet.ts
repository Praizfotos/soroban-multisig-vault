import {
  isConnected,
  getAddress,
  signTransaction,
  getNetwork,
} from '@stellar/freighter-api';

export interface WalletState {
  isConnected: boolean;
  publicKey: string | null;
  network: string | null;
}

export class FreighterWallet {
  static async checkConnection(): Promise<boolean> {
    try {
      const result = await isConnected();
      return result.isConnected;
    } catch (error) {
      console.error('Error checking Freighter connection:', error);
      return false;
    }
  }

  static async connect(): Promise<string> {
    try {
      const result = await getAddress();
      return result.address;
    } catch (error) {
      console.error('Error connecting to Freighter:', error);
      throw new Error('Failed to connect to Freighter wallet');
    }
  }

  static async getNetwork(): Promise<string> {
    try {
      const result = await getNetwork();
      return result.network;
    } catch (error) {
      console.error('Error getting network:', error);
      throw new Error('Failed to get network from Freighter');
    }
  }

  static async signTransaction(xdr: string, network: string): Promise<string> {
    try {
      const result = await signTransaction(xdr, {
        networkPassphrase: network === 'TESTNET' 
          ? 'Test SDF Network ; September 2015'
          : 'Public Global Stellar Network ; September 2015',
      });
      return result.signedTxXdr;
    } catch (error) {
      console.error('Error signing transaction:', error);
      throw new Error('Failed to sign transaction');
    }
  }

  static async getWalletState(): Promise<WalletState> {
    try {
      const connected = await this.checkConnection();
      if (!connected) {
        return {
          isConnected: false,
          publicKey: null,
          network: null,
        };
      }

      const addressResult = await getAddress();
      const networkResult = await getNetwork();

      return {
        isConnected: true,
        publicKey: addressResult.address,
        network: networkResult.network,
      };
    } catch (error) {
      console.error('Error getting wallet state:', error);
      return {
        isConnected: false,
        publicKey: null,
        network: null,
      };
    }
  }
}
