# API Reference

REST API documentation for Soroban Multi-Sig Treasury Vault backend.

## Base URL

- **Development**: `http://localhost:3001`
- **Testnet**: `https://api-testnet.yourdomain.com`
- **Mainnet**: `https://api.yourdomain.com`

## Authentication

Currently no authentication required. All data is public on-chain.

## Rate Limiting

- 100 requests per 15 minutes per IP
- Headers included in response:
  - `X-RateLimit-Limit`
  - `X-RateLimit-Remaining`
  - `X-RateLimit-Reset`

## Endpoints

### Health Check

#### GET /health

Check API server status.

**Response**:
```json
{
  "status": "ok",
  "timestamp": "2024-06-30T18:00:00.000Z"
}
```

### Treasuries

#### GET /api/treasuries

List all treasuries.

**Response**:
```json
[
  {
    "id": "treasury-dao-main",
    "name": "DAO Main Treasury",
    "creator": "GCEXAMPLE...",
    "threshold": 3,
    "created_at": "2024-01-01T00:00:00.000Z",
    "paused": false,
    "metadata": {}
  }
]
```

#### GET /api/treasuries/:id

Get treasury details.

**Parameters**:
- `id` (string) - Treasury identifier

**Response**:
```json
{
  "id": "treasury-dao-main",
  "name": "DAO Main Treasury",
  "creator": "GCEXAMPLE...",
  "threshold": 3,
  "created_at": "2024-01-01T00:00:00.000Z",
  "paused": false,
  "metadata": {}
}
```

**Error**: 404 if treasury not found

#### GET /api/treasuries/creator/:address

Get treasuries by creator address.

**Parameters**:
- `address` (string) - Stellar address

**Response**:
```json
[
  {
    "id": "treasury-dao-main",
    "name": "DAO Main Treasury",
    ...
  }
]
```

#### GET /api/treasuries/:id/signers

Get treasury signers.

**Response**:
```json
[
  {
    "address": "GCEXAMPLE...",
    "added_at": "2024-01-01T00:00:00.000Z",
    "active": true
  }
]
```

#### GET /api/treasuries/:id/balance

Get treasury balances.

**Response**:
```json
[
  {
    "asset": "native",
    "balance": "1000000000000"
  }
]
```

### Proposals

#### GET /api/proposals

List proposals.

**Query Parameters**:
- `treasury_id` (optional) - Filter by treasury
- `status` (optional) - Filter by status

**Response**:
```json
[
  {
    "id": "proposal-transfer-1",
    "treasury_id": "treasury-dao-main",
    "proposal_type": "transfer",
    "proposer": "GCEXAMPLE...",
    "status": "pending",
    "approvals": 2,
    "rejections": 0,
    "created_at": "2024-01-01T00:00:00.000Z",
    "expires_at": "2024-01-08T00:00:00.000Z",
    "executed": false,
    "payload": {
      "recipient": "GCRECIPIENT...",
      "asset": "native",
      "amount": "1000"
    }
  }
]
```

#### GET /api/proposals/:id

Get proposal details.

**Response**:
```json
{
  "id": "proposal-transfer-1",
  "treasury_id": "treasury-dao-main",
  "proposal_type": "transfer",
  "proposer": "GCEXAMPLE...",
  "status": "approved",
  "approvals": 3,
  "rejections": 0,
  "created_at": "2024-01-01T00:00:00.000Z",
  "expires_at": "2024-01-08T00:00:00.000Z",
  "executed": true,
  "executed_at": "2024-01-02T00:00:00.000Z",
  "payload": {}
}
```

#### GET /api/proposals/:id/votes

Get votes for a proposal.

**Response**:
```json
[
  {
    "proposal_id": "proposal-transfer-1",
    "voter": "GCEXAMPLE...",
    "vote_type": "approve",
    "timestamp": "2024-01-01T00:00:00.000Z"
  }
]
```

### Events

#### GET /api/events

Get event history.

**Query Parameters**:
- `treasury_id` (optional) - Filter by treasury
- `limit` (optional) - Max results (default 100)

**Response**:
```json
[
  {
    "event_type": "treasury_created",
    "treasury_id": "treasury-dao-main",
    "data": {
      "name": "DAO Main Treasury",
      "threshold": 3
    },
    "ledger": 1000,
    "transaction_hash": "abc123...",
    "timestamp": "2024-01-01T00:00:00.000Z"
  }
]
```

### Statistics

#### GET /api/stats

Get platform statistics.

**Response**:
```json
{
  "treasuries": 10,
  "proposals": 45,
  "votes": 123
}
```

## Error Responses

All errors follow this format:

```json
{
  "error": "Error type",
  "message": "Detailed error message"
}
```

**Status Codes**:
- `400` - Bad Request
- `404` - Not Found
- `429` - Too Many Requests
- `500` - Internal Server Error

## Examples

### JavaScript/TypeScript

```typescript
import axios from 'axios';

const API_URL = 'http://localhost:3001';

// Get all treasuries
const treasuries = await axios.get(`${API_URL}/api/treasuries`);

// Get specific treasury
const treasury = await axios.get(`${API_URL}/api/treasuries/treasury-dao-main`);

// Get proposals for treasury
const proposals = await axios.get(
  `${API_URL}/api/proposals?treasury_id=treasury-dao-main`
);
```

### cURL

```bash
# Get treasuries
curl http://localhost:3001/api/treasuries

# Get proposal
curl http://localhost:3001/api/proposals/proposal-transfer-1

# Get events
curl http://localhost:3001/api/events?treasury_id=treasury-dao-main
```

### Python

```python
import requests

API_URL = "http://localhost:3001"

# Get all treasuries
response = requests.get(f"{API_URL}/api/treasuries")
treasuries = response.json()

# Get specific proposal
response = requests.get(f"{API_URL}/api/proposals/proposal-transfer-1")
proposal = response.json()
```
