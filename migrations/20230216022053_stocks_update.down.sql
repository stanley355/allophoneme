-- Add down migration script here
ALTER TABLE stocks 
DROP COLUMN sector,
DROP COLUMN subsector,
DROP COLUMN report_currency,
DROP COLUMN stock_outstanding;
