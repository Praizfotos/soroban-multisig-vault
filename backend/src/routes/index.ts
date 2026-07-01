import { Router } from 'express';
import { query } from '../db/pool';

export const voteRoutes = Router();
export const eventRoutes = Router();
export const statsRoutes = Router();

// Vote routes
voteRoutes.get('/', async (req, res, next) => {
  try {
    const result = await query('SELECT * FROM votes ORDER BY timestamp DESC LIMIT 100');
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Event routes
eventRoutes.get('/', async (req, res, next) => {
  try {
    const { treasury_id } = req.query;
    let sql = 'SELECT * FROM events WHERE 1=1';
    const params: any[] = [];
    
    if (treasury_id) {
      params.push(treasury_id);
      sql += ` AND treasury_id = $${params.length}`;
    }
    
    sql += ' ORDER BY timestamp DESC LIMIT 100';
    
    const result = await query(sql, params);
    res.json(result.rows);
  } catch (error) {
    next(error);
  }
});

// Stats routes
statsRoutes.get('/', async (req, res, next) => {
  try {
    const treasuryCount = await query('SELECT COUNT(*) as count FROM treasuries');
    const proposalCount = await query('SELECT COUNT(*) as count FROM proposals');
    const voteCount = await query('SELECT COUNT(*) as count FROM votes');
    
    res.json({
      treasuries: parseInt(treasuryCount.rows[0].count),
      proposals: parseInt(proposalCount.rows[0].count),
      votes: parseInt(voteCount.rows[0].count),
    });
  } catch (error) {
    next(error);
  }
});
