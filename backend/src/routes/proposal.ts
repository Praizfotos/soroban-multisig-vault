import { Router } from 'express';
import { query } from '../db/pool';

export const proposalRoutes = Router();

// Get all proposals
proposalRoutes.get('/', async (req, res, next) => {
  try {
    const { treasury_id, status } = req.query;
    
    let sql = 'SELECT * FROM proposals WHERE 1=1';
    const params: any[] = [];
    
    if (treasury_id) {
      params.push(treasury_id);
      sql += ` AND treasury_id = $${params.length}`;
    }
    
    if (status) {
      params.push(status);
      sql += ` AND status = $${params.length}`;
    }
    
    sql += ' ORDER BY created_at DESC';
    
    const result = await query(sql, params);
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Get proposal by ID
proposalRoutes.get('/:id', async (req, res, next) => {
  try {
    const { id } = req.params;
    const result = await query('SELECT * FROM proposals WHERE id = $1', [id]);
    
    if (result.rows.length === 0) {
      return res.status(404).json({ error: 'Proposal not found' });
    }
    
    res.json(result.rows[0]);
  } catch (error) {
    next(error);
  }
});

// Get proposal votes
proposalRoutes.get('/:id/votes', async (req, res, next) => {
  try {
    const { id } = req.params;
    const result = await query(
      'SELECT * FROM votes WHERE proposal_id = $1 ORDER BY timestamp DESC',
      [id]
    );
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});
