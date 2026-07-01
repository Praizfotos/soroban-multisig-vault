-- Seed data for development and testing

-- Insert sample treasuries
INSERT INTO treasuries (id, name, creator, threshold, paused, metadata) VALUES
('treasury-dao-main', 'DAO Main Treasury', 'GCEXAMPLE1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456', 3, false, '{"description": "Primary DAO treasury", "type": "DAO"}'),
('treasury-startup', 'Startup Operations Fund', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', 2, false, '{"description": "Operating expenses", "type": "Company"}'),
('treasury-grants', 'Grant Distribution Fund', 'GCEXAMPLE3456789012ABCDEFGHIJKLMNOPQRSTUVWXYZ345678', 4, false, '{"description": "Community grants", "type": "Foundation"}')
ON CONFLICT DO NOTHING;

-- Insert sample signers
INSERT INTO signers (treasury_id, address, active) VALUES
('treasury-dao-main', 'GCEXAMPLE1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456', true),
('treasury-dao-main', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', true),
('treasury-dao-main', 'GCEXAMPLE3456789012ABCDEFGHIJKLMNOPQRSTUVWXYZ345678', true),
('treasury-dao-main', 'GCEXAMPLE4567890123ABCDEFGHIJKLMNOPQRSTUVWXYZ456789', true),
('treasury-dao-main', 'GCEXAMPLE5678901234ABCDEFGHIJKLMNOPQRSTUVWXYZ567890', true),
('treasury-startup', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', true),
('treasury-startup', 'GCEXAMPLE6789012345ABCDEFGHIJKLMNOPQRSTUVWXYZ678901', true),
('treasury-startup', 'GCEXAMPLE7890123456ABCDEFGHIJKLMNOPQRSTUVWXYZ789012', true),
('treasury-grants', 'GCEXAMPLE3456789012ABCDEFGHIJKLMNOPQRSTUVWXYZ345678', true),
('treasury-grants', 'GCEXAMPLE8901234567ABCDEFGHIJKLMNOPQRSTUVWXYZ890123', true),
('treasury-grants', 'GCEXAMPLE9012345678ABCDEFGHIJKLMNOPQRSTUVWXYZ901234', true),
('treasury-grants', 'GCEXAMPLE0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ012345', true),
('treasury-grants', 'GCEXAMPLE1234567890BCDEFGHIJKLMNOPQRSTUVWXYZ123456', true)
ON CONFLICT DO NOTHING;

-- Insert sample proposals
INSERT INTO proposals (id, treasury_id, proposal_type, proposer, status, payload, expires_at) VALUES
('proposal-transfer-1', 'treasury-dao-main', 'transfer', 'GCEXAMPLE1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456', 'pending', 
 '{"recipient": "GCRECIPIENT123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ", "asset": "native", "amount": "1000"}', 
 NOW() + INTERVAL '7 days'),
('proposal-add-signer-1', 'treasury-startup', 'add_signer', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', 'approved',
 '{"new_signer": "GCNEWSIGNER1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ"}',
 NOW() + INTERVAL '5 days'),
('proposal-threshold-1', 'treasury-grants', 'update_threshold', 'GCEXAMPLE3456789012ABCDEFGHIJKLMNOPQRSTUVWXYZ345678', 'pending',
 '{"new_threshold": 3}',
 NOW() + INTERVAL '10 days')
ON CONFLICT DO NOTHING;

-- Insert sample votes
INSERT INTO votes (proposal_id, voter, vote_type) VALUES
('proposal-transfer-1', 'GCEXAMPLE1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456', 'approve'),
('proposal-transfer-1', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', 'approve'),
('proposal-add-signer-1', 'GCEXAMPLE2345678901ABCDEFGHIJKLMNOPQRSTUVWXYZ234567', 'approve'),
('proposal-add-signer-1', 'GCEXAMPLE6789012345ABCDEFGHIJKLMNOPQRSTUVWXYZ678901', 'approve')
ON CONFLICT DO NOTHING;

-- Insert sample events
INSERT INTO events (event_type, treasury_id, data, ledger, transaction_hash) VALUES
('treasury_created', 'treasury-dao-main', '{"name": "DAO Main Treasury", "threshold": 3}', 1000, 'abc123def456'),
('treasury_created', 'treasury-startup', '{"name": "Startup Operations Fund", "threshold": 2}', 1001, 'def456ghi789'),
('proposal_created', 'treasury-dao-main', '{"proposal_id": "proposal-transfer-1", "type": "transfer"}', 1050, 'ghi789jkl012'),
('vote_cast', 'treasury-dao-main', '{"proposal_id": "proposal-transfer-1", "voter": "GCEXAMPLE1", "approve": true}', 1051, 'jkl012mno345')
ON CONFLICT DO NOTHING;

-- Update analytics
UPDATE analytics SET metric_value = 3 WHERE metric_name = 'total_treasuries';
UPDATE analytics SET metric_value = 3 WHERE metric_name = 'total_proposals';
UPDATE analytics SET metric_value = 4 WHERE metric_name = 'total_votes';
