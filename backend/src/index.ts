import express from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { logger } from './utils/logger';
import { treasuryRoutes } from './routes/treasury';
import { proposalRoutes } from './routes/proposal';
import { voteRoutes, eventRoutes, statsRoutes } from './routes/index';
import { errorHandler } from './middleware/errorHandler';
import { startEventIndexer } from './services/indexer';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Routes
app.use('/api/treasuries', treasuryRoutes);
app.use('/api/proposals', proposalRoutes);
app.use('/api/votes', voteRoutes);
app.use('/api/events', eventRoutes);
app.use('/api/stats', statsRoutes);

// Error handling
app.use(errorHandler);

// Start server
app.listen(PORT, () => {
  logger.info(`Backend server running on port ${PORT}`);
  
  // Start event indexer
  if (process.env.ENABLE_INDEXER === 'true') {
    startEventIndexer();
    logger.info('Event indexer started');
  }
});

export default app;
