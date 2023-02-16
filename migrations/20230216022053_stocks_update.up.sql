-- Add up migration script here
ALTER TABLE stocks 
ADD COLUMN sector VARCHAR(100) NOT NULL,
ADD COLUMN subsector VARCHAR(100) NOT NULL,
ADD COLUMN report_currency VARCHAR DEFAULT 'IDR', 
ADD COLUMN stock_outstanding INTEGER NOT NULL;
