# Architecture Guide

## Overview

Soroban Multi-Sig Treasury Vault is a comprehensive treasury management protocol built on Stellar's Soroban smart contract platform. This document describes the system architecture, component interactions, and design decisions.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend Layer                        │
│  ┌────────────┐  ┌──────────────┐  ┌─────────────────────┐ │
│  │  Next.js   │  │    Wallet    │  │   UI Components     │ │
│  │  React App │◄─┤ Integration  │  │   (Shadcn/Tailwind) │ │
│  └─────┬──────┘  └──────┬───────┘  └─────────────────────┘ │
│        │                │                                    │
└────────┼────────────────┼────────────────────────────────────┘
         │                │
         │ API Calls      │ Freighter Wallet
         │                │
         ▼                ▼
┌─────────────────────────────────────────────────────────────┐
│                       Backend Layer                          │
│  ┌──────────────┐  ┌─────────────┐  ┌──────────────────┐  │
│  │ Express API  │  │   Event     │  │   PostgreSQL     │  │
│  │   Server     │◄─┤  Indexer    │──┤    Database      │  │
│  └──────┬───────┘  └──────┬──────┘  └──────────────────┘  │
│         │                  │                                 │
└─────────┼──────────────────┼─────────────────────────────────┘
          │                  │
          │ RPC Calls        │ Event Polling
          │                  │
          ▼                  ▼
┌─────────────────────────────────────────────────────────────┐
│                    Stellar/Soroban Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐ │
│  │   Treasury   │  │  Governance  │  │    Registry      │ │
│  │   Contract   │  │   Contract   │  │    Contract      │ │
│  └──────────────┘  └──────────────┘  └──────────────────┘ │
│                                                              │
│                 Soroban Smart Contract Platform              │
│                        Stellar Network                       │
└─────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### Smart Contract Layer

#### Treasury Contract
**Purpose**: Core multi-signature treasury logic

**Responsibilities**:
- Treasury initialization and configuration
- Multi-signature authorization
- Proposal creation and management
- Vote tracking and threshold validation
- Automatic execution when threshold met
- Asset custody and transfers
- Emergency pause/resume controls

**Key Functions**:
- `initialize()` - Create new treasury
- `create_proposal()` - Submit governance proposal
- `vote()` - Cast vote on proposal
- `execute_proposal()` - Execute approved proposal
- `deposit()` - Add funds to treasury
- `get_treasury()` - Query treasury state
- `get_proposal()` - Query proposal details

**Storage Structure**:
- Persistent storage for treasuries, proposals, votes
- Balance tracking per asset
- Signer management

#### Governance Contract
**Purpose**: Voting configuration and quorum management

**Responsibilities**:
- Governance parameter configuration
- Vote tallying
- Quorum threshold checks
- Voting period management

**Key Functions**:
- `initialize()` - Set governance config
- `record_vote()` - Log vote with power
- `get_vote_counts()` - Aggregate votes
- `check_quorum()` - Verify quorum met

#### Registry Contract
**Purpose**: Treasury discovery and metadata

**Responsibilities**:
- Treasury registration
- Metadata management
- Global treasury directory
- Statistics tracking

**Key Functions**:
- `register_treasury()` - Add to registry
- `get_treasury()` - Lookup treasury info
- `get_all_treasuries()` - List all treasuries
- `get_treasuries_by_creator()` - Filter by creator
- `get_stats()` - Global statistics

### Backend Layer

#### Express API Server
RESTful API for frontend integration.

**Endpoints**:
```
GET  /api/treasuries              # List all treasuries
GET  /api/treasuries/:id          # Get treasury details
GET  /api/treasuries/creator/:addr # Get by creator
GET  /api/proposals               # List proposals
GET  /api/proposals/:id           # Get proposal details
GET  /api/proposals/:id/votes     # Get proposal votes
GET  /api/events                  # Event history
GET  /api/stats                   # Platform statistics
```

#### Event Indexer Service
Monitors Soroban events and syncs to database.

**Process**:
1. Poll Soroban RPC for new events
2. Parse event XDR data
3. Normalize and store in PostgreSQL
4. Trigger notifications if configured
5. Update analytics

**Indexed Events**:
- Treasury created/paused/resumed
- Proposal created/executed
- Vote cast
- Signer added/removed
- Threshold updated
- Deposit received

### Database Layer

#### PostgreSQL Schema

**Core Tables**:
- `treasuries` - Treasury configurations
- `signers` - Authorized signer tracking
- `proposals` - All proposals
- `votes` - Individual votes
- `events` - Event history
- `balances` - Asset balances
- `notifications` - Notification queue
- `analytics` - Metrics and stats

**Indexes**:
- Optimized for treasury/proposal lookups
- Creator filtering
- Time-based queries
- Event type filtering

### Frontend Layer

#### Next.js Application

**Pages**:
- `/` - Landing page
- `/dashboard` - User dashboard
- `/treasuries` - Treasury list
- `/treasuries/[id]` - Treasury details
- `/proposals` - Proposal list
- `/proposals/[id]` - Proposal details
- `/docs` - Documentation

**Key Features**:
- Server-side rendering for SEO
- Client-side state with Zustand
- React Query for data fetching
- Optimistic UI updates
- Real-time wallet integration

#### Wallet Integration

**Supported Wallets**:
- Freighter (primary)
- WalletConnect-compatible wallets

**Capabilities**:
- Connect/disconnect wallet
- Sign transactions
- Query account state
- Network switching

## Data Flow

### Creating a Treasury

```
1. User → Frontend: Fill treasury form
2. Frontend → Wallet: Request signature
3. Wallet → Frontend: Signed transaction
4. Frontend → Soroban: Submit transaction
5. Soroban → Contract: Execute initialize()
6. Contract → Soroban: Emit TreasuryCreated event
7. Soroban → Indexer: Event detected
8. Indexer → Database: Store treasury
9. Database → Frontend: Updated data
```

### Proposal Lifecycle

```
1. Signer creates proposal
   └─ Contract validates: Is authorized signer?
   └─ Contract stores proposal with Pending status

2. Signers vote on proposal
   └─ Contract validates: Haven't voted before?
   └─ Contract records vote
   └─ Contract checks threshold

3. Threshold reached?
   └─ Yes: Auto-execute or wait for manual execution
   └─ No: Keep in Pending state

4. Proposal execution
   └─ Contract validates: Is approved?
   └─ Contract validates: Not expired?
   └─ Contract executes action (transfer, add signer, etc.)
   └─ Contract emits ProposalExecuted event
```

## Security Model

### Authorization Layers

1. **Wallet Level**: User must sign with private key
2. **Contract Level**: Must be authorized signer
3. **Proposal Level**: Threshold must be met
4. **Expiration**: Proposals have time limits

### Attack Vectors & Mitigations

| Attack | Mitigation |
|--------|------------|
| Unauthorized access | Signer validation on every action |
| Replay attacks | Proposal IDs and vote tracking |
| Double voting | Unique constraint on (proposal, voter) |
| Expired proposals | Ledger-based expiration checks |
| Threshold bypass | Validation before execution |
| Rogue admin | No single-signer control |

### Emergency Controls

**Pause Mechanism**:
- Any authorized signer can propose pause
- Requires threshold approval
- When paused: No transfers, no non-resume proposals
- Resume also requires threshold approval

## Scalability Considerations

### Contract Storage

- Uses persistent storage for critical data
- Temporary storage for intermediate states
- Efficient key structures for lookups

### Database Optimization

- Indexed queries for common patterns
- Partitioning for large event tables
- Connection pooling
- Read replicas for scaling reads

### Frontend Performance

- Server-side rendering for initial load
- Code splitting and lazy loading
- Asset optimization
- CDN distribution

## Deployment Architecture

### Development
```
Local Docker → PostgreSQL + Backend + Frontend
             → Mock Soroban testnet
```

### Staging/Testnet
```
Cloud Provider → Managed PostgreSQL
              → Container instances (Backend)
              → Static hosting (Frontend)
              → Soroban Testnet
```

### Production/Mainnet
```
Cloud Provider → HA PostgreSQL cluster
              → Auto-scaling backend
              → CDN + Static hosting
              → Soroban Mainnet
              → Monitoring & alerts
```

## Technology Stack

### Smart Contracts
- **Language**: Rust
- **SDK**: Soroban SDK 21.0.0
- **Platform**: Stellar Soroban

### Backend
- **Runtime**: Node.js 20
- **Framework**: Express
- **Database**: PostgreSQL 16
- **ORM**: Raw SQL (pg driver)
- **Language**: TypeScript

### Frontend
- **Framework**: Next.js 14
- **Language**: TypeScript
- **UI**: Tailwind CSS + Shadcn
- **State**: Zustand
- **Data Fetching**: TanStack Query

### DevOps
- **Containerization**: Docker
- **Orchestration**: Docker Compose
- **CI/CD**: GitHub Actions
- **Deployment**: Cloud-agnostic scripts

## Design Decisions

### Why Multi-Contract Architecture?

**Decision**: Separate Treasury, Governance, and Registry contracts

**Rationale**:
- Separation of concerns
- Independent upgrade paths
- Reusable governance logic
- Clearer security boundaries

### Why Persistent Storage?

**Decision**: Use persistent storage vs temporary

**Rationale**:
- Treasury data must survive beyond session
- Proposals need long-term tracking
- Audit trail requirements

### Why Auto-Execute on Threshold?

**Decision**: Execute proposals when threshold met

**Rationale**:
- Better UX - no extra transaction
- Atomic approval + execution
- Reduces gas costs
- Prevents timing attacks

### Why PostgreSQL?

**Decision**: PostgreSQL over NoSQL

**Rationale**:
- ACID guarantees
- Complex queries (joins, aggregations)
- Mature ecosystem
- Well-understood operations

## Future Enhancements

- [ ] Multi-asset treasury support
- [ ] Weighted voting (different signer powers)
- [ ] Timelock for execution delay
- [ ] Recurring payment schedules
- [ ] Treasury templates
- [ ] Mobile app
- [ ] Hardware wallet support
- [ ] Multi-chain support
