import { Router } from 'express';
import { query } from '../db/pool';
import { logger } from '../utils/logger';

export const treasuryRoutes = Router();

// Get all treasuries
treasuryRoutes.get('/', async (req, res, next) => {
  try {
    const result = await query('SELECT * FROM treasuries ORDER BY created_at DESC');
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Get treasury by ID
treasuryRoutes.get('/:id', async (req, res, next) => {
  try {
    const { id } = req.params;
    const result = await query('SELECT * FROM treasuries WHERE id = $1', [id]);
    
    if (result.rows.length === 0) {
      return res.status(404).json({ error: 'Treasury not found' });
    }
    
    res.json(result.rows[0]);
  } catch (error) {
    next(error);
  }
});

// Get treasuries by creator
treasuryRoutes.get('/creator/:address', async (req, res, next) => {
  try {
    const { address } = req.params;
    const result = await query(
      'SELECT * FROM treasuries WHERE creator = $1 ORDER BY created_at DESC',
      [address]
    );
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Get treasury signers
treasuryRoutes.get('/:id/signers', async (req, res, next) => {
  try {
    const { id } = req.params;
    const result = await query(
      'SELECT * FROM signers WHERE treasury_id = $1 ORDER BY added_at DESC',
      [id]
    );
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Get treasury balance
treasuryRoutes.get('/:id/balance', async (req, res, next) => {
  try {
    const { id } = req.params;
    const result = await query(
      'SELECT asset, SUM(amount) as balance FROM events WHERE treasury_id = $1 AND event_type IN ($2, $3) GROUP BY asset',
      [id, 'deposit', 'transfer']
    );
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});
