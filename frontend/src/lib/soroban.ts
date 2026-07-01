import { 
  Contract, 
  SorobanRpc, 
  TransactionBuilder, 
  Networks,
  BASE_FEE,
  xdr,
  Address,
  nativeToScVal
} from '@stellar/stellar-sdk';

const RPC_URLS = {
  testnet: 'https://soroban-testnet.stellar.org',
  mainnet: 'https://soroban-mainnet.stellar.org',
};

const NETWORK_PASSPHRASES = {
  testnet: Networks.TESTNET,
  mainnet: Networks.PUBLIC,
};

export class TreasuryClient {
  private contract: Contract;
  private server: SorobanRpc.Server;
  private networkPassphrase: string;

  constructor(contractId: string, network: 'testnet' | 'mainnet' = 'testnet') {
    this.contract = new Contract(contractId);
    this.server = new SorobanRpc.Server(RPC_URLS[network]);
    this.networkPassphrase = NETWORK_PASSPHRASES[network];
  }

  async initializeTreasury(
    sourceAccount: string,
    treasuryId: string,
    name: string,
    signers: string[],
    threshold: number
  ): Promise<string> {
    const account = await this.server.getAccount(sourceAccount);

    const operation = this.contract.call(
      'initialize',
      nativeToScVal(treasuryId, { type: 'string' }),
      nativeToScVal(name, { type: 'string' }),
      nativeToScVal(sourceAccount, { type: 'address' }),
      nativeToScVal(signers.map(s => new Address(s)), { type: 'vec' }),
      nativeToScVal(threshold, { type: 'u32' })
    );

    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    const prepared = await this.server.prepareTransaction(transaction);
    return prepared.toXDR();
  }

  async createProposal(
    sourceAccount: string,
    treasuryId: string,
    proposalId: string,
    proposalType: any,
    expirationLedger: number
  ): Promise<string> {
    const account = await this.server.getAccount(sourceAccount);

    const operation = this.contract.call(
      'create_proposal',
      nativeToScVal(treasuryId, { type: 'string' }),
      nativeToScVal(proposalId, { type: 'string' }),
      nativeToScVal(sourceAccount, { type: 'address' }),
      proposalType,
      nativeToScVal(expirationLedger, { type: 'u32' })
    );

    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    const prepared = await this.server.prepareTransaction(transaction);
    return prepared.toXDR();
  }

  async vote(
    sourceAccount: string,
    proposalId: string,
    approve: boolean
  ): Promise<string> {
    const account = await this.server.getAccount(sourceAccount);

    const operation = this.contract.call(
      'vote',
      nativeToScVal(proposalId, { type: 'string' }),
      nativeToScVal(sourceAccount, { type: 'address' }),
      nativeToScVal(approve, { type: 'bool' })
    );

    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    const prepared = await this.server.prepareTransaction(transaction);
    return prepared.toXDR();
  }

  async getTreasury(treasuryId: string): Promise<any> {
    // Use a placeholder account for simulation
    const account = await this.server.getAccount('GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF');

    const operation = this.contract.call(
      'get_treasury',
      nativeToScVal(treasuryId, { type: 'string' })
    );

    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    const result = await this.server.simulateTransaction(transaction);
    
    if (SorobanRpc.Api.isSimulationSuccess(result)) {
      return result.result;
    }

    throw new Error('Failed to get treasury');
  }

  async getProposal(proposalId: string): Promise<any> {
    // Use a placeholder account for simulation
    const account = await this.server.getAccount('GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF');

    const operation = this.contract.call(
      'get_proposal',
      nativeToScVal(proposalId, { type: 'string' })
    );

    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(operation)
      .setTimeout(30)
      .build();

    const result = await this.server.simulateTransaction(transaction);
    
    if (SorobanRpc.Api.isSimulationSuccess(result)) {
      return result.result;
    }

    throw new Error('Failed to get proposal');
  }

  async submitTransaction(signedXdr: string): Promise<any> {
    const transaction = TransactionBuilder.fromXDR(signedXdr, this.networkPassphrase);
    const result = await this.server.sendTransaction(transaction as any);
    
    if (result.status === 'PENDING') {
      // Poll for result
      let getResponse = await this.server.getTransaction(result.hash);
      
      while (getResponse.status === SorobanRpc.Api.GetTransactionStatus.NOT_FOUND) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        getResponse = await this.server.getTransaction(result.hash);
      }

      if (getResponse.status === SorobanRpc.Api.GetTransactionStatus.SUCCESS) {
        return getResponse;
      }
    }

    throw new Error('Transaction failed');
  }
}

export const createTreasuryClient = () => {
  const contractId = process.env.NEXT_PUBLIC_TREASURY_CONTRACT_ID;
  const network = (process.env.NEXT_PUBLIC_SOROBAN_NETWORK || 'testnet') as 'testnet' | 'mainnet';

  if (!contractId) {
    throw new Error('Treasury contract ID not configured');
  }

  return new TreasuryClient(contractId, network);
};
