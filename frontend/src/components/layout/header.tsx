'use client';

import Link from 'next/link';
import { useWallet } from '@/hooks/useWallet';
import { Button } from '@/components/ui/button';
import { Wallet, LogOut } from 'lucide-react';

export function Header() {
  const { connected, publicKey, connect, disconnect, loading } = useWallet();

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 4)}...${addr.slice(-4)}`;
  };

  return (
    <header className="border-b">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-8">
            <Link href="/" className="text-xl font-bold">
              Multi-Sig Vault
            </Link>
            <nav className="hidden md:flex gap-6">
              <Link href="/dashboard" className="text-muted-foreground hover:text-foreground">
                Dashboard
              </Link>
              <Link href="/treasuries" className="text-muted-foreground hover:text-foreground">
                Treasuries
              </Link>
              <Link href="/proposals" className="text-muted-foreground hover:text-foreground">
                Proposals
              </Link>
              <Link href="/docs" className="text-muted-foreground hover:text-foreground">
                Docs
              </Link>
            </nav>
          </div>

          <div>
            {connected && publicKey ? (
              <div className="flex items-center gap-2">
                <span className="text-sm text-muted-foreground">
                  {formatAddress(publicKey)}
                </span>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={disconnect}
                >
                  <LogOut className="h-4 w-4 mr-2" />
                  Disconnect
                </Button>
              </div>
            ) : (
              <Button
                onClick={connect}
                disabled={loading}
              >
                <Wallet className="h-4 w-4 mr-2" />
                {loading ? 'Connecting...' : 'Connect Wallet'}
              </Button>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}
