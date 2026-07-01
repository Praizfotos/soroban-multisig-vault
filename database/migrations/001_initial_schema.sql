-- Create treasuries table
CREATE TABLE IF NOT EXISTS treasuries (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    creator VARCHAR(56) NOT NULL,
    threshold INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    paused BOOLEAN DEFAULT FALSE,
    metadata JSONB,
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_treasuries_creator ON treasuries(creator);
CREATE INDEX idx_treasuries_created_at ON treasuries(created_at DESC);

-- Create signers table
CREATE TABLE IF NOT EXISTS signers (
    id SERIAL PRIMARY KEY,
    treasury_id VARCHAR(255) NOT NULL REFERENCES treasuries(id) ON DELETE CASCADE,
    address VARCHAR(56) NOT NULL,
    added_at TIMESTAMP NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMP,
    active BOOLEAN DEFAULT TRUE,
    UNIQUE(treasury_id, address)
);

CREATE INDEX idx_signers_treasury_id ON signers(treasury_id);
CREATE INDEX idx_signers_address ON signers(address);
CREATE INDEX idx_signers_active ON signers(active);

-- Create proposals table
CREATE TABLE IF NOT EXISTS proposals (
    id VARCHAR(255) PRIMARY KEY,
    treasury_id VARCHAR(255) NOT NULL REFERENCES treasuries(id) ON DELETE CASCADE,
    proposal_type VARCHAR(50) NOT NULL,
    proposer VARCHAR(56) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    executed_at TIMESTAMP,
    executed BOOLEAN DEFAULT FALSE
);

CREATE INDEX idx_proposals_treasury_id ON proposals(treasury_id);
CREATE INDEX idx_proposals_proposer ON proposals(proposer);
CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposals_created_at ON proposals(created_at DESC);

-- Create votes table
CREATE TABLE IF NOT EXISTS votes (
    id SERIAL PRIMARY KEY,
    proposal_id VARCHAR(255) NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    voter VARCHAR(56) NOT NULL,
    vote_type VARCHAR(10) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(proposal_id, voter)
);

CREATE INDEX idx_votes_proposal_id ON votes(proposal_id);
CREATE INDEX idx_votes_voter ON votes(voter);
CREATE INDEX idx_votes_timestamp ON votes(timestamp DESC);

-- Create events table
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    treasury_id VARCHAR(255),
    proposal_id VARCHAR(255),
    address VARCHAR(56),
    data JSONB NOT NULL,
    ledger INTEGER NOT NULL,
    transaction_hash VARCHAR(64) NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_event_type ON events(event_type);
CREATE INDEX idx_events_treasury_id ON events(treasury_id);
CREATE INDEX idx_events_proposal_id ON events(proposal_id);
CREATE INDEX idx_events_address ON events(address);
CREATE INDEX idx_events_ledger ON events(ledger);
CREATE INDEX idx_events_timestamp ON events(timestamp DESC);

-- Create balances table
CREATE TABLE IF NOT EXISTS balances (
    treasury_id VARCHAR(255) NOT NULL REFERENCES treasuries(id) ON DELETE CASCADE,
    asset VARCHAR(56) NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (treasury_id, asset)
);

CREATE INDEX idx_balances_treasury_id ON balances(treasury_id);

-- Create notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id SERIAL PRIMARY KEY,
    treasury_id VARCHAR(255) NOT NULL REFERENCES treasuries(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL,
    recipient VARCHAR(255) NOT NULL,
    channel VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    payload JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    sent_at TIMESTAMP
);

CREATE INDEX idx_notifications_treasury_id ON notifications(treasury_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);

-- Create analytics table
CREATE TABLE IF NOT EXISTS analytics (
    id SERIAL PRIMARY KEY,
    metric_name VARCHAR(100) NOT NULL,
    metric_value NUMERIC NOT NULL,
    dimensions JSONB,
    recorded_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_analytics_metric_name ON analytics(metric_name);
CREATE INDEX idx_analytics_recorded_at ON analytics(recorded_at DESC);

-- Insert initial analytics
INSERT INTO analytics (metric_name, metric_value, dimensions) VALUES
('total_treasuries', 0, '{}'),
('total_proposals', 0, '{}'),
('total_votes', 0, '{}')
ON CONFLICT DO NOTHING;

-- Create function to update timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers
CREATE TRIGGER update_treasuries_updated_at
    BEFORE UPDATE ON treasuries
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_balances_updated_at
    BEFORE UPDATE ON balances
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
