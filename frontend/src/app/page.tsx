import Link from "next/link";
import { ArrowRight, Shield, Users, Vote, Lock } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export default function Home() {
  return (
    <div className="space-y-16">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-5xl font-bold tracking-tight">
          Secure Treasury Management
          <br />
          <span className="text-primary">Built on Stellar</span>
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Multi-signature treasury infrastructure for DAOs, startups, and communities.
          No single wallet can move funds without collective approval.
        </p>
        <div className="flex gap-4 justify-center">
          <Link href="/dashboard">
            <Button size="lg">
              Launch App
              <ArrowRight className="ml-2 h-4 w-4" />
            </Button>
          </Link>
          <Link href="/docs">
            <Button size="lg" variant="outline">
              Read Docs
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <CardHeader>
            <Shield className="h-12 w-12 text-primary mb-2" />
            <CardTitle>Multi-Signature Security</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Configure N-of-M approval thresholds. Every action requires multiple authorized signatures.
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <Vote className="h-12 w-12 text-primary mb-2" />
            <CardTitle>Proposal Governance</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Create proposals for transfers, signer changes, and threshold updates. Vote transparently on-chain.
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <Users className="h-12 w-12 text-primary mb-2" />
            <CardTitle>Flexible Signer Management</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Add or remove authorized signers through governance. Update approval thresholds as your org evolves.
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <Lock className="h-12 w-12 text-primary mb-2" />
            <CardTitle>Emergency Controls</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Built-in pause mechanism for security incidents. Resume operations through multi-sig approval.
            </CardDescription>
          </CardContent>
        </Card>
      </section>

      {/* Use Cases */}
      <section className="space-y-6">
        <h2 className="text-3xl font-bold text-center">Built For</h2>
        <div className="grid md:grid-cols-3 gap-6">
          <Card>
            <CardHeader>
              <CardTitle>DAOs & Communities</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription>
                Decentralized treasury management with transparent on-chain governance and collective decision-making.
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Startups & Companies</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription>
                Secure company funds with multi-party oversight. Prevent single points of failure and unauthorized access.
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Grant Programs</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription>
                Transparent fund distribution with accountability. Track every grant approval and disbursement on-chain.
              </CardDescription>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* CTA Section */}
      <section className="text-center space-y-6 py-12 bg-muted rounded-lg">
        <h2 className="text-3xl font-bold">Ready to Secure Your Treasury?</h2>
        <p className="text-muted-foreground max-w-xl mx-auto">
          Deploy your multi-sig treasury vault in minutes. Connect your wallet to get started.
        </p>
        <Link href="/dashboard">
          <Button size="lg">
            Get Started
            <ArrowRight className="ml-2 h-4 w-4" />
          </Button>
        </Link>
      </section>
    </div>
  );
}
