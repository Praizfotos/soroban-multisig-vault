# Database Schema

PostgreSQL database schema for Soroban Multi-Sig Treasury Vault.

## Setup

### Local Development

```bash
# Create database
createdb soroban_multisig

# Run migrations
psql soroban_multisig < migrations/001_initial_schema.sql

# Load seed data (optional)
psql soroban_multisig < seeds/seed.sql
```

### Docker

```bash
docker-compose up -d postgres
```

## Schema

### Tables

#### treasuries
Stores treasury metadata and configuration.

- `id` - Treasury unique identifier
- `name` - Treasury name
- `creator` - Stellar address of creator
- `threshold` - Approval threshold
- `created_at` - Creation timestamp
- `paused` - Emergency pause status
- `metadata` - Additional JSON metadata

#### signers
Tracks authorized signers for each treasury.

- `treasury_id` - Foreign key to treasuries
- `address` - Stellar address of signer
- `added_at` - When signer was added
- `removed_at` - When signer was removed (if applicable)
- `active` - Current active status

#### proposals
All treasury proposals.

- `id` - Proposal unique identifier
- `treasury_id` - Foreign key to treasuries
- `proposal_type` - Type (transfer, add_signer, etc.)
- `proposer` - Address of proposal creator
- `status` - Current status (pending, approved, rejected, executed)
- `payload` - Proposal-specific data
- `expires_at` - Expiration timestamp
- `executed` - Execution status

#### votes
Individual votes on proposals.

- `proposal_id` - Foreign key to proposals
- `voter` - Address of voter
- `vote_type` - approve or reject
- `timestamp` - When vote was cast

#### events
All contract events indexed from Soroban.

- `event_type` - Type of event
- `treasury_id` - Related treasury (if applicable)
- `data` - Event-specific data
- `ledger` - Ledger sequence number
- `transaction_hash` - Transaction hash
- `timestamp` - Event timestamp

#### balances
Current treasury asset balances.

- `treasury_id` - Foreign key to treasuries
- `asset` - Asset identifier
- `balance` - Current balance

#### notifications
Notification queue for webhooks and emails.

- `treasury_id` - Related treasury
- `notification_type` - Type of notification
- `recipient` - Email or webhook URL
- `channel` - Email, Discord, or Slack
- `status` - Delivery status
- `payload` - Notification data

## Migrations

Migrations are located in `migrations/` directory and should be run in order.

## Maintenance

### Backup

```bash
pg_dump soroban_multisig > backup_$(date +%Y%m%d).sql
```

### Restore

```bash
psql soroban_multisig < backup_20240101.sql
```
